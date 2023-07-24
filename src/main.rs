use clap::{Parser,CommandFactory};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Extract xzp
    #[arg(short,long)]
    extract: bool,
    /// Create xzp
    #[arg(short,long)]
    create: bool,

    /// Base Directoy where source files are found
    #[arg(value_name = "IN_FILE")]
    input_file: Option<PathBuf>,
    /// XZP out
    #[arg(value_name = "OUT_FILE")]
    output_file: Option<PathBuf>,

}

fn main() {
    let args = Args::parse();
    let mut cmd = Args::command();
    cmd.print_help();
}
