use crate::types::TaxonId;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaxTreeError {
    #[error("missing taxonomy dump file: {0}")]
    MissingDumpFile(PathBuf),

    #[error("invalid dump row in {file} at line {line}: {message}")]
    InvalidDumpRow {
        file: String,
        line: usize,
        message: String,
    },

    #[error("failed to read file {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write file {path}: {source}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to encode index: {0}")]
    EncodeIndex(#[from] bincode::Error),

    #[error("failed to serialize json: {0}")]
    SerializeJson(#[from] serde_json::Error),

    #[error("unsupported index version {found}; expected {expected}")]
    UnsupportedIndexVersion { found: u32, expected: u32 },

    #[error("invalid index header")]
    InvalidIndexHeader,

    #[error("taxon name not found: {0}")]
    NameNotFound(String),

    #[error("taxon name is ambiguous: {name}; candidates: {candidates:?}")]
    AmbiguousName {
        name: String,
        candidates: Vec<TaxonId>,
    },

    #[error("taxid not found: {0}")]
    TaxIdNotFound(TaxonId),

    #[error("input is empty")]
    EmptyInput,

    #[error("unsupported output format {format} for command {command}")]
    UnsupportedFormat { command: String, format: String },
}
