use crate::types::{TaxonRecord, TaxonomyEdge};

pub fn records(records: &[TaxonRecord]) -> String {
    let mut out = String::from("name\ttaxid\trank\n");
    for record in records {
        out.push_str(&format!(
            "{}\t{}\t{}\n",
            record.name, record.id, record.rank
        ));
    }
    out
}

pub fn edges(edges: &[TaxonomyEdge]) -> String {
    let mut out = String::from("child_taxid\tchild_name\tparent_taxid\tparent_name\trank\n");
    for edge in edges {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            edge.child.id, edge.child.name, edge.parent.id, edge.parent.name, edge.child.rank
        ));
    }
    out
}
