mod error;
mod leetcode;
mod templates;

use clap::{Parser, ValueEnum};
use std::{fmt, path::PathBuf, process};

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let title = args.title;
    let lang = args.language;
    let out_dir = args.output_dir;
    let force = args.force;

    let question = match leetcode::get_question(&title).await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Something went wrong while getting question : '{e}'");
            process::exit(1);
        }
    };

    /*
    if !out_dir.exists() {
        if !force {
            eprintln!(
                "Output directory '{}' does not exist and '--force' was not used! Please use '--force' to automatically create output pathway.",
                out_dir.to_string_lossy()
            );
            process::exit(1);
        }

        // TODO : Create output dir
    }
    */

    println!(
        "title = '{title}'\nlanguage = '{lang}'\noutput directory = '{out_dir:?}'\nforce = '{force}'\nquestion : \n\t'title={}'\n\t'content={}'\n\t'test_cases={}'",
        question.title, question.content, question.example_testcases
    );
}

#[derive(Parser, Debug)]
#[command(name = "leetlocal", version, about = "Scaffolding to bring leetcode questions local", long_about = None)]
struct CliArgs {
    /// Query leetcode for question based upon it's id
    #[arg(short, long, required = true)]
    title: String,

    /// The language you want to solve the problem in
    #[arg(short, long, required = true, value_enum)]
    language: Language,

    /// Destination directory where the question files will be created
    /// - **must be a directory**
    /// - path is relative to current working directory (where the CLI is being called from)
    #[arg(short, long, required = true)]
    output_dir: PathBuf,

    /// Force creation of output directory
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
#[value(rename_all = "verbatim")]
pub enum Language {
    JavaScript,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::JavaScript => write!(f, "JavaScript"),
        }
    }
}
