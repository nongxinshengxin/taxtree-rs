use std::fs;
use std::path::Path;
use taxtree_core::format::{json, tsv};
use taxtree_core::{OutputFormat, TaxTreeError, TaxonRecord, TaxonomyIndex};

pub fn run(
    index_path: &Path,
    name: Option<&str>,
    input: Option<&Path>,
    format: OutputFormat,
    out: Option<&Path>,
) -> Result<(), TaxTreeError> {
    let index = TaxonomyIndex::load(index_path)?;
    let names = read_names(name, input)?;
    let mut records = Vec::new();
    for item in names {
        records.push(index.rank_by_name(&item)?);
    }

    let rendered = render_records(&records, format)?;
    write_or_print(out, &rendered)
}

fn read_names(name: Option<&str>, input: Option<&Path>) -> Result<Vec<String>, TaxTreeError> {
    if let Some(name) = name {
        return Ok(vec![name.to_string()]);
    }

    let Some(input) = input else {
        return Err(TaxTreeError::EmptyInput);
    };

    let content = fs::read_to_string(input).map_err(|source| TaxTreeError::ReadFile {
        path: input.to_path_buf(),
        source,
    })?;
    let names: Vec<_> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();

    if names.is_empty() {
        Err(TaxTreeError::EmptyInput)
    } else {
        Ok(names)
    }
}

fn render_records(records: &[TaxonRecord], format: OutputFormat) -> Result<String, TaxTreeError> {
    match format {
        OutputFormat::Tsv => Ok(tsv::records(records)),
        OutputFormat::Json => json::records(records),
        OutputFormat::Newick => Err(TaxTreeError::UnsupportedFormat {
            command: "rank".to_string(),
            format: "newick".to_string(),
        }),
    }
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
