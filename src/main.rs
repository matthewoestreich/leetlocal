mod leetcode_api;
mod templates;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
}

#[derive(Parser, Debug)]
#[command(name = "leetlocal", version, about = "Scaffolding to bring leetcode questions local", long_about = None)]
struct CliArgs {
    /// Query leetcode for question based upon it's id
    #[arg(short, long, conflicts_with = "id", required_unless_present = "id")]
    title: Option<String>,

    /// Query leetcode for question based upon it's title
    #[arg(
        short,
        long,
        conflicts_with = "title",
        required_unless_present = "title"
    )]
    id: Option<u32>,

    /// The language you want to solve the problem in
    #[arg(short, long, required = true, value_enum)]
    language: Language,

    /// Destination pathway where the question files will be created
    #[arg(short, long, required = true)]
    output: PathBuf,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Language {
    JavaScript,
}
