mod error;
mod language;
mod leetcode;

pub(crate) use crate::{error::Error, language::Language};

use clap::Parser;
use std::{fs, path::PathBuf};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("ERROR : {e}");
    }
}

// This is its own fn bc i want to print the formatted error message, not the debug formatted error message.
async fn run() -> Result<(), Error> {
    let args = CliArgs::parse();

    let problem = args.problem;
    let lang = args.language;
    let out_dir = args.output_dir;
    let force = args.force;
    let description_file_name = args.description_file_name;
    let code_file_name = args.code_file_name;

    let question = leetcode::get_question(&problem).await?;

    let question_snippet = question
        .get_snippet(&lang)
        .ok_or(Error::UnsupportedLanguage(lang))?;

    if out_dir.exists() && out_dir.is_file() {
        return Err(Error::OutputDirectoryNotDirectory);
    }
    if !out_dir.exists() && !force {
        return Err(Error::OutputDirectoryNotFound {
            output_path: out_dir.to_string_lossy().to_string(),
        });
    }

    let mut readme_path = out_dir.clone();
    readme_path.push(format!("{description_file_name}.md"));

    if readme_path.exists() && !force {
        return Err(Error::DescriptionFileExists {
            description_file_name: format!("{description_file_name}.md"),
        });
    }

    let mut code_path = out_dir.clone();
    code_path.push(&code_file_name);
    code_path.set_extension(lang.file_extension());

    if code_path.exists() && !force {
        return Err(Error::CodeFileExists {
            code_file_name,
            code_file_extension: lang.file_extension(),
        });
    }

    write_file(readme_path, &question.create_readme_data())?;
    write_file(code_path, &question_snippet)?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "leetlocal", version, about = "Scaffolding to bring leetcode questions local", long_about = None)]
struct CliArgs {
    /// Query leetcode for question based upon it's title slug
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

    /// The file name for the problem description, saved as markdown, so no file extension needed.
    #[arg(short, long, default_value = "README")]
    description_file_name: String,

    /// The file name for the problem code. Automatically saved with appropriate file extension
    /// based on '--language', so no need to provide an extension.
    #[arg(short, long, default_value = "main")]
    code_file_name: String,

    /// Force creation of output directory and/or overwrite existing files
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn write_file(path: PathBuf, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}
