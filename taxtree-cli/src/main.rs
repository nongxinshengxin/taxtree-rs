mod commands;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use taxtree_core::{OutputFormat, TaxonId};

#[derive(Debug, Parser)]
#[command(name = "taxtree")]
#[command(about = "Build and query taxonomy trees from NCBI Taxonomy dumps")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Index {
        #[arg(long)]
        dump_dir: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    Rank {
        #[arg(long)]
        index: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        input: Option<PathBuf>,
        #[arg(long, value_enum, default_value = "tsv")]
        format: CliFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Tree {
        #[arg(long)]
        index: PathBuf,
        #[arg(long)]
        input: Option<PathBuf>,
        #[arg(long)]
        taxid: Vec<TaxonId>,
        #[arg(long, value_enum, default_value = "newick")]
        format: CliFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Lineage {
        #[arg(long)]
        index: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        taxid: Option<TaxonId>,
        #[arg(long, default_value_t = 1)]
        depth: usize,
        #[arg(long, value_enum, default_value = "tsv")]
        format: CliFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliFormat {
    Tsv,
    Json,
    Newick,
}

impl From<CliFormat> for OutputFormat {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Tsv => OutputFormat::Tsv,
            CliFormat::Json => OutputFormat::Json,
            CliFormat::Newick => OutputFormat::Newick,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Index { dump_dir, out } => commands::index::run(&dump_dir, &out),
        Command::Rank {
            index,
            name,
            input,
            format,
            out,
        } => commands::rank::run(
            &index,
            name.as_deref(),
            input.as_deref(),
            format.into(),
            out.as_deref(),
        ),
        Command::Tree {
            index,
            input,
            taxid,
            format,
            out,
        } => commands::tree::run(
            &index,
            input.as_deref(),
            &taxid,
            format.into(),
            out.as_deref(),
        ),
        Command::Lineage {
            index,
            name,
            taxid,
            depth,
            format,
            out,
        } => commands::lineage::run(
            &index,
            name.as_deref(),
            taxid,
            depth,
            format.into(),
            out.as_deref(),
        ),
    };

    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
