use std::path::Path;
use taxtree_core::dump::{parse_names_dump, parse_nodes_dump};
use taxtree_core::format::{json, newick, tsv};
use taxtree_core::{TaxonNode, TaxonomyIndex, TaxonomyTree};
use tempfile::tempdir;

fn fixture_dir() -> &'static Path {
    Path::new("tests/fixtures/taxonomy")
}

#[test]
fn domain_types_can_be_constructed() {
    let node = TaxonNode {
        id: 9606,
        parent_id: 9605,
        rank: "species".to_string(),
    };

    let tree = TaxonomyTree {
        root_id: 1,
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    assert_eq!(node.id, 9606);
    assert_eq!(tree.root_id, 1);
}

#[test]
fn parses_scientific_names_from_ncbi_dump() {
    let names = parse_names_dump(&fixture_dir().join("names.dmp")).unwrap();

    assert_eq!(names.get(&9606).unwrap(), "Homo sapiens");
    assert_eq!(names.get(&562).unwrap(), "Escherichia coli");
    assert_eq!(names.get(&1).unwrap(), "root");
}

#[test]
fn parses_nodes_from_ncbi_dump() {
    let nodes = parse_nodes_dump(&fixture_dir().join("nodes.dmp")).unwrap();

    let human = nodes.get(&9606).unwrap();
    assert_eq!(human.parent_id, 9605);
    assert_eq!(human.rank, "species");
}

#[test]
fn builds_index_from_fixture_dump() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();

    assert_eq!(index.record_by_id(9606).unwrap().name, "Homo sapiens");
    assert_eq!(index.record_by_id(562).unwrap().rank, "species");
}

#[test]
fn saves_and_loads_index() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let dir = tempdir().unwrap();
    let path = dir.path().join("taxonomy.ttidx");

    index.save(&path).unwrap();
    let loaded = TaxonomyIndex::load(&path).unwrap();

    assert_eq!(loaded.record_by_id(9606).unwrap().name, "Homo sapiens");
}

#[test]
fn resolves_rank_by_name() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let record = index.rank_by_name("Homo sapiens").unwrap();

    assert_eq!(record.id, 9606);
    assert_eq!(record.rank, "species");
}

#[test]
fn reports_ambiguous_names() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let error = index.resolve_name("Duplicate name").unwrap_err();

    assert!(error.to_string().contains("ambiguous"));
    assert!(error.to_string().contains("1000"));
    assert!(error.to_string().contains("1001"));
}

#[test]
fn walks_ancestors_from_species_to_root() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let ancestors = index.ancestors(9606).unwrap();
    let ids: Vec<_> = ancestors.iter().map(|record| record.id).collect();

    assert_eq!(ids, vec![9606, 9605, 9604, 9443, 7711, 33208, 2759, 1]);
}

#[test]
fn expands_descendants_by_depth() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let edges = index.descendants(9443, 2).unwrap();
    let child_ids: Vec<_> = edges.iter().map(|edge| edge.child.id).collect();

    assert_eq!(child_ids, vec![9604, 9598, 9605]);
}

#[test]
fn builds_merged_tree_from_multiple_taxa() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let tree = index.build_tree(&[9606, 9598]).unwrap();
    let node_ids: Vec<_> = tree.nodes.iter().map(|record| record.id).collect();
    let edges: Vec<_> = tree
        .edges
        .iter()
        .map(|edge| (edge.child.id, edge.parent.id))
        .collect();

    assert_eq!(tree.root_id, 1);
    assert!(node_ids.contains(&9606));
    assert!(node_ids.contains(&9598));
    assert!(node_ids.contains(&9604));
    assert!(edges.contains(&(9606, 9605)));
    assert!(edges.contains(&(9598, 9604)));
}

#[test]
fn renders_rank_records_as_tsv_and_json() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let record = index.rank_by_name("Homo sapiens").unwrap();

    let tsv_output = tsv::records(std::slice::from_ref(&record));
    let json_output = json::records(&[record]).unwrap();

    assert!(tsv_output.contains("name\ttaxid\trank"));
    assert!(tsv_output.contains("Homo sapiens\t9606\tspecies"));
    assert!(json_output.contains("\"name\":\"Homo sapiens\""));
}

#[test]
fn renders_tree_as_newick() {
    let index = TaxonomyIndex::build_from_dump(fixture_dir()).unwrap();
    let tree = index.build_tree(&[9606, 9598]).unwrap();
    let output = newick::tree(&tree);

    assert!(output.ends_with(';'));
    assert!(output.contains("Homo_sapiens"));
    assert!(output.contains("Pan_troglodytes"));
}
