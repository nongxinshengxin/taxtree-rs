use crate::dump::{parse_names_dump, parse_nodes_dump};
use crate::error::TaxTreeError;
use crate::types::{TaxonId, TaxonNode, TaxonRecord};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const MAGIC: &[u8] = b"TAXTREE_IDX";
const VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyIndex {
    nodes: HashMap<TaxonId, TaxonNode>,
    names_by_id: HashMap<TaxonId, String>,
    ids_by_name: HashMap<String, Vec<TaxonId>>,
    children_by_id: HashMap<TaxonId, Vec<TaxonId>>,
}

impl TaxonomyIndex {
    pub fn build_from_dump(dump_dir: impl AsRef<Path>) -> Result<Self, TaxTreeError> {
        let dump_dir = dump_dir.as_ref();
        let names_path = dump_dir.join("names.dmp");
        let nodes_path = dump_dir.join("nodes.dmp");
        ensure_file(&names_path)?;
        ensure_file(&nodes_path)?;

        let names_by_id = parse_names_dump(&names_path)?;
        let nodes = parse_nodes_dump(&nodes_path)?;

        let mut ids_by_name: HashMap<String, Vec<TaxonId>> = HashMap::new();
        for (id, name) in &names_by_id {
            ids_by_name.entry(name.clone()).or_default().push(*id);
        }

        let mut children_by_id: HashMap<TaxonId, Vec<TaxonId>> = HashMap::new();
        for node in nodes.values() {
            if node.id != node.parent_id {
                children_by_id
                    .entry(node.parent_id)
                    .or_default()
                    .push(node.id);
            }
        }

        for ids in ids_by_name.values_mut() {
            ids.sort_unstable();
        }
        for ids in children_by_id.values_mut() {
            ids.sort_unstable();
        }

        Ok(Self {
            nodes,
            names_by_id,
            ids_by_name,
            children_by_id,
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), TaxTreeError> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend(bincode::serialize(self)?);

        let path = path.as_ref();
        fs::write(path, bytes).map_err(|source| TaxTreeError::WriteFile {
            path: path.to_path_buf(),
            source,
        })
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, TaxTreeError> {
        let path = path.as_ref();
        let bytes = fs::read(path).map_err(|source| TaxTreeError::ReadFile {
            path: path.to_path_buf(),
            source,
        })?;

        let header_len = MAGIC.len() + 4;
        if bytes.len() < header_len || &bytes[..MAGIC.len()] != MAGIC {
            return Err(TaxTreeError::InvalidIndexHeader);
        }

        let version_offset = MAGIC.len();
        let found = u32::from_le_bytes(
            bytes[version_offset..version_offset + 4]
                .try_into()
                .expect("version slice has four bytes"),
        );
        if found != VERSION {
            return Err(TaxTreeError::UnsupportedIndexVersion {
                found,
                expected: VERSION,
            });
        }

        Ok(bincode::deserialize(&bytes[header_len..])?)
    }

    pub fn record_by_id(&self, id: TaxonId) -> Result<TaxonRecord, TaxTreeError> {
        let node = self.nodes.get(&id).ok_or(TaxTreeError::TaxIdNotFound(id))?;
        let name = self
            .names_by_id
            .get(&id)
            .cloned()
            .unwrap_or_else(|| id.to_string());

        Ok(TaxonRecord {
            id,
            name,
            rank: node.rank.clone(),
            parent_id: node.parent_id,
        })
    }

    pub(crate) fn ids_for_name(&self, name: &str) -> Option<&[TaxonId]> {
        self.ids_by_name.get(name).map(Vec::as_slice)
    }

    pub(crate) fn children_of(&self, id: TaxonId) -> &[TaxonId] {
        self.children_by_id
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

fn ensure_file(path: &Path) -> Result<(), TaxTreeError> {
    if path.is_file() {
        Ok(())
    } else {
        Err(TaxTreeError::MissingDumpFile(PathBuf::from(path)))
    }
}
