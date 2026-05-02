use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../taxtree-core/tests/fixtures/taxonomy")
}

#[test]
fn prints_help() {
    let mut cmd = Command::cargo_bin("taxtree").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn indexes_and_queries_rank() {
    let dir = tempdir().unwrap();
    let fixture_dir = fixture_dir();
    let index_path = dir.path().join("taxonomy.ttidx");

    Command::cargo_bin("taxtree")
        .unwrap()
        .args([
            "index",
            "--dump-dir",
            fixture_dir.to_str().unwrap(),
            "--out",
            index_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("taxtree")
        .unwrap()
        .args([
            "rank",
            "--index",
            index_path.to_str().unwrap(),
            "--name",
            "Homo sapiens",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Homo sapiens\t9606\tspecies"));
}

#[test]
fn writes_newick_tree() {
    let dir = tempdir().unwrap();
    let fixture_dir = fixture_dir();
    let index_path = dir.path().join("taxonomy.ttidx");
    let taxa_path = dir.path().join("taxa.txt");
    let tree_path = dir.path().join("tree.nwk");
    fs::write(&taxa_path, "Homo sapiens\nPan troglodytes\n").unwrap();

    Command::cargo_bin("taxtree")
        .unwrap()
        .args([
            "index",
            "--dump-dir",
            fixture_dir.to_str().unwrap(),
            "--out",
            index_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("taxtree")
        .unwrap()
        .args([
            "tree",
            "--index",
            index_path.to_str().unwrap(),
            "--input",
            taxa_path.to_str().unwrap(),
            "--format",
            "newick",
            "--out",
            tree_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let newick = fs::read_to_string(tree_path).unwrap();
    assert!(newick.contains("Homo_sapiens"));
    assert!(newick.contains("Pan_troglodytes"));
    assert!(newick.ends_with(";\n") || newick.ends_with(';'));
}

#[test]
fn exits_nonzero_for_ambiguous_name() {
    let dir = tempdir().unwrap();
    let fixture_dir = fixture_dir();
    let index_path = dir.path().join("taxonomy.ttidx");

    Command::cargo_bin("taxtree")
        .unwrap()
        .args([
            "index",
            "--dump-dir",
            fixture_dir.to_str().unwrap(),
            "--out",
            index_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("taxtree")
        .unwrap()
        .args([
            "rank",
            "--index",
            index_path.to_str().unwrap(),
            "--name",
            "Duplicate name",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ambiguous"));
}
