use std::path::Path;
use taxtree_core::{TaxTreeError, TaxonomyIndex};

pub fn run(dump_dir: &Path, out: &Path) -> Result<(), TaxTreeError> {
    let index = TaxonomyIndex::build_from_dump(dump_dir)?;
    index.save(out)?;
    Ok(())
}
