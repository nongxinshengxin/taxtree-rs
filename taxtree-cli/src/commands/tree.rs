use std::fs;
use std::path::Path;
use taxtree_core::format::{json, newick, tsv};
use taxtree_core::{OutputFormat, TaxTreeError, TaxonId, TaxonomyIndex};

pub fn run(
    index_path: &Path,
    names_input: Option<&Path>,
    taxids: &[TaxonId],
    format: OutputFormat,
    out: Option<&Path>,
) -> Result<(), TaxTreeError> {
    let index = TaxonomyIndex::load(index_path)?;
    let mut ids = taxids.to_vec();

    if let Some(input) = names_input {
        let content = fs::read_to_string(input).map_err(|source| TaxTreeError::ReadFile {
            path: input.to_path_buf(),
            source,
        })?;
        for line in content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            ids.push(index.resolve_name(line)?);
        }
    }

    if ids.is_empty() {
        return Err(TaxTreeError::EmptyInput);
    }

    let tree = index.build_tree(&ids)?;
    let rendered = match format {
        OutputFormat::Tsv => tsv::edges(&tree.edges),
        OutputFormat::Json => json::tree(&tree)?,
        OutputFormat::Newick => newick::tree(&tree),
    };

    write_or_print(out, &rendered)
}

fn write_or_print(out: Option<&Path>, rendered: &str) -> Result<(), TaxTreeError> {
    if let Some(out) = out {
        fs::write(out, rendered).map_err(|source| TaxTreeError::WriteFile {
            path: out.to_path_buf(),
            source,
        })
    } else {
        println!("{rendered}");
        Ok(())
    }
}
