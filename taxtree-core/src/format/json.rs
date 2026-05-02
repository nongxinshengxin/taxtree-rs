use crate::error::TaxTreeError;
use crate::types::{TaxonRecord, TaxonomyEdge, TaxonomyTree};
use serde::Serialize;

#[derive(Serialize)]
struct RecordsOutput<'a> {
    records: &'a [TaxonRecord],
}

#[derive(Serialize)]
struct EdgesOutput<'a> {
    edges: &'a [TaxonomyEdge],
}

pub fn records(records: &[TaxonRecord]) -> Result<String, TaxTreeError> {
    serde_json::to_string(&RecordsOutput { records }).map_err(TaxTreeError::from)
}

pub fn edges(edges: &[TaxonomyEdge]) -> Result<String, TaxTreeError> {
    serde_json::to_string(&EdgesOutput { edges }).map_err(TaxTreeError::from)
}

pub fn tree(tree: &TaxonomyTree) -> Result<String, TaxTreeError> {
    serde_json::to_string(tree).map_err(TaxTreeError::from)
}
