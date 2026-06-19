use crate::error::TaxTreeError;
use crate::types::{TaxonId, TaxonNode};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn parse_names_dump(path: &Path) -> Result<HashMap<TaxonId, String>, TaxTreeError> {
    let content = fs::read_to_string(path).map_err(|source| TaxTreeError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut names = HashMap::new();
    for (idx, line) in content.lines().enumerate() {
        let fields = split_dump_line(line);
        if fields.len() < 4 {
            return Err(TaxTreeError::InvalidDumpRow {
                file: path.display().to_string(),
                line: idx + 1,
                message: "expected at least 4 fields".to_string(),
            });
        }

        if fields[3] != "scientific name" {
            continue;
        }

        let id = parse_taxid(fields[0], path, idx + 1)?;
        names.insert(id, fields[1].to_string());
    }

    Ok(names)
}

pub fn parse_nodes_dump(path: &Path) -> Result<HashMap<TaxonId, TaxonNode>, TaxTreeError> {
    let content = fs::read_to_string(path).map_err(|source| TaxTreeError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut nodes = HashMap::new();
    for (idx, line) in content.lines().enumerate() {
        let fields = split_dump_line(line);
        if fields.len() < 3 {
            return Err(TaxTreeError::InvalidDumpRow {
                file: path.display().to_string(),
                line: idx + 1,
                message: "expected at least 3 fields".to_string(),
            });
        }

        let id = parse_taxid(fields[0], path, idx + 1)?;
        let parent_id = parse_taxid(fields[1], path, idx + 1)?;
        nodes.insert(
            id,
            TaxonNode {
                id,
                parent_id,
                rank: fields[2].to_string(),
            },
        );
    }

    Ok(nodes)
}

fn split_dump_line(line: &str) -> Vec<&str> {
    line.split('|').map(str::trim).collect()
}

fn parse_taxid(raw: &str, path: &Path, line: usize) -> Result<TaxonId, TaxTreeError> {
    raw.parse::<TaxonId>()
        .map_err(|_| TaxTreeError::InvalidDumpRow {
            file: path.display().to_string(),
            line,
            message: format!("invalid taxid '{raw}'"),
        })
}
