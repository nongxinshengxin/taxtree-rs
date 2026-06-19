use serde::{Deserialize, Serialize};

pub type TaxonId = u64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxonNode {
    pub id: TaxonId,
    pub parent_id: TaxonId,
    pub rank: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxonRecord {
    pub id: TaxonId,
    pub name: String,
    pub rank: String,
    pub parent_id: TaxonId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxonomyEdge {
    pub child: TaxonRecord,
    pub parent: TaxonRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxonomyTree {
    pub root_id: TaxonId,
    pub nodes: Vec<TaxonRecord>,
    pub edges: Vec<TaxonomyEdge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Tsv,
    Json,
    Newick,
}
