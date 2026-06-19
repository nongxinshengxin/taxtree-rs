use crate::error::TaxTreeError;
use crate::index::TaxonomyIndex;
use crate::types::{TaxonId, TaxonRecord, TaxonomyEdge};
use std::collections::VecDeque;

impl TaxonomyIndex {
    pub fn resolve_name(&self, name: &str) -> Result<TaxonId, TaxTreeError> {
        let ids = self
            .ids_for_name(name)
            .ok_or_else(|| TaxTreeError::NameNotFound(name.to_string()))?;

        match ids {
            [id] => Ok(*id),
            many => Err(TaxTreeError::AmbiguousName {
                name: name.to_string(),
                candidates: many.to_vec(),
            }),
        }
    }

    pub fn rank_by_name(&self, name: &str) -> Result<TaxonRecord, TaxTreeError> {
        let id = self.resolve_name(name)?;
        self.record_by_id(id)
    }

    pub fn ancestors(&self, id: TaxonId) -> Result<Vec<TaxonRecord>, TaxTreeError> {
        let mut records = Vec::new();
        let mut current = id;

        loop {
            let record = self.record_by_id(current)?;
            let parent_id = record.parent_id;
            records.push(record);

            if current == parent_id {
                break;
            }

            current = parent_id;
        }

        Ok(records)
    }

    pub fn descendants(
        &self,
        id: TaxonId,
        depth: usize,
    ) -> Result<Vec<TaxonomyEdge>, TaxTreeError> {
        self.record_by_id(id)?;

        let mut edges = Vec::new();
        let mut queue = VecDeque::from([(id, 0usize)]);

        while let Some((parent_id, level)) = queue.pop_front() {
            if level >= depth {
                continue;
            }

            let parent = self.record_by_id(parent_id)?;
            for child_id in self.children_of(parent_id) {
                let child = self.record_by_id(*child_id)?;
                edges.push(TaxonomyEdge {
                    child: child.clone(),
                    parent: parent.clone(),
                });
                queue.push_back((*child_id, level + 1));
            }
        }

        Ok(edges)
    }
}
