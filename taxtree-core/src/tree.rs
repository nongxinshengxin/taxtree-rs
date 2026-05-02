use crate::error::TaxTreeError;
use crate::index::TaxonomyIndex;
use crate::types::{TaxonId, TaxonRecord, TaxonomyEdge, TaxonomyTree};
use std::collections::{HashMap, HashSet};

impl TaxonomyIndex {
    pub fn build_tree(&self, ids: &[TaxonId]) -> Result<TaxonomyTree, TaxTreeError> {
        if ids.is_empty() {
            return Err(TaxTreeError::EmptyInput);
        }

        let mut nodes_by_id: HashMap<TaxonId, TaxonRecord> = HashMap::new();
        let mut edge_ids: HashSet<(TaxonId, TaxonId)> = HashSet::new();

        for id in ids {
            let ancestors = self.ancestors(*id)?;
            for record in &ancestors {
                nodes_by_id
                    .entry(record.id)
                    .or_insert_with(|| record.clone());
                if record.id != record.parent_id {
                    edge_ids.insert((record.id, record.parent_id));
                }
            }
        }

        let mut nodes: Vec<_> = nodes_by_id.values().cloned().collect();
        nodes.sort_by_key(|record| record.id);

        let mut edges = Vec::new();
        let mut sorted_edge_ids: Vec<_> = edge_ids.into_iter().collect();
        sorted_edge_ids.sort_unstable();
        for (child_id, parent_id) in sorted_edge_ids {
            let child = nodes_by_id
                .get(&child_id)
                .cloned()
                .ok_or(TaxTreeError::TaxIdNotFound(child_id))?;
            let parent = nodes_by_id
                .get(&parent_id)
                .cloned()
                .ok_or(TaxTreeError::TaxIdNotFound(parent_id))?;
            edges.push(TaxonomyEdge { child, parent });
        }

        Ok(TaxonomyTree {
            root_id: 1,
            nodes,
            edges,
        })
    }
}
