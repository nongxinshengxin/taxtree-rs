use std::fs;
use std::path::Path;
use taxtree_core::format::{json, tsv};
use taxtree_core::{OutputFormat, TaxTreeError, TaxonId, TaxonomyIndex};

pub fn run(
    index_path: &Path,
    name: Option<&str>,
    taxid: Option<TaxonId>,
    depth: usize,
    format: OutputFormat,
    out: Option<&Path>,
) -> Result<(), TaxTreeError> {
    let index = TaxonomyIndex::load(index_path)?;
    let id = match (taxid, name) {
        (Some(id), _) => id,
        (None, Some(name)) => index.resolve_name(name)?,
        (None, None) => return Err(TaxTreeError::EmptyInput),
    };

    let edges = index.descendants(id, depth)?;
    let rendered = match format {
        OutputFormat::Tsv => tsv::edges(&edges),
        OutputFormat::Json => json::edges(&edges)?,
        OutputFormat::Newick => {
            return Err(TaxTreeError::UnsupportedFormat {
                command: "lineage".to_string(),
                format: "newick".to_string(),
            })
        }
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
        print!("{rendered}");
        Ok(())
    }
}
