mod error;
mod language;
mod leetcode;

pub(crate) use crate::{error::Error, language::Language};

use clap::Parser;
use std::{fs, path::PathBuf, process};

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let problem = args.problem;
    let lang = args.language;
    let out_dir = args.output_dir;
    let force = args.force;

    let question = match leetcode::get_question(&problem).await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Something went wrong while getting question : '{e}'");
            process::exit(1);
        }
    };

    let question_snippet = question.get_snippet(&lang);
    if question_snippet.is_none() {
        eprintln!("Question does not have support for language '{lang}'");
        process::exit(1);
    }

    if !out_dir.exists() && !force {
        eprintln!(
            "Output directory '{}' does not exist and '--force' was not used! Please use '--force' to automatically create output pathway.",
            out_dir.to_string_lossy()
        );
        process::exit(1);
    }
    if out_dir.exists() && out_dir.is_file() {
        eprintln!("Output directory must be to a directory, not a file!");
        process::exit(1);
    }

    let mut readme_path = out_dir.clone();
    readme_path.push("README.md");

    if readme_path.exists() && !force {
        eprintln!(
            "README.md already exists at output directory. Please use '--force' to overwrite files!"
        );
        process::exit(1);
    }

    let mut code_path = out_dir.clone();
    code_path.push("main");
    code_path.set_extension(lang.file_extension());

    if code_path.exists() && !force {
        eprintln!(
            "main.{} already exists at output directory. Please use '--force' to overwrite files!",
            lang.file_extension()
        );
        process::exit(1);
    }

    write_file(readme_path, &question.create_readme_data()).unwrap_or_else(|e| {
        eprintln!("Something went wrong writing README! {e:?}");
        process::exit(1);
    });

    let code_data = question_snippet.expect("to be already validated").code;
    write_file(code_path, &code_data).unwrap_or_else(|e| {
        eprintln!("Something went wrong writing main! {e:?}");
        process::exit(1);
    });
}

#[derive(Parser, Debug)]
#[command(name = "leetlocal", version, about = "Scaffolding to bring leetcode questions local", long_about = None)]
struct CliArgs {
    /// Query leetcode for question based upon it's id
    #[arg(short, long, required = true)]
    problem: String,

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

fn write_file(path: PathBuf, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}
