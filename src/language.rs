use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, Debug, ValueEnum)]
#[value(rename_all = "verbatim")]
pub enum Language {
    Cpp,
    Java,
    Python3,
    Python,
    JavaScript,
    TypeScript,
    CSharp,
    C,
    Go,
    Kotlin,
    Swift,
    Rust,
    Ruby,
    Php,
    Dart,
    Scala,
    Elixir,
    Erlang,
    Racket,
    Cangjie,
    Bash,
    React,
    MySQL,
    #[allow(clippy::upper_case_acronyms)]
    MSSQL,
    PostgreSQL,
    OracleSQL,
    Pandas,
}

impl Language {
    pub fn slug(&self) -> String {
        match self {
            Language::JavaScript => "javascript".to_string(),
            Language::Cpp => "cpp".to_string(),
            Language::Java => "java".to_string(),
            Language::Python3 => "python3".to_string(),
            Language::Python => "python".to_string(),
            Language::TypeScript => "typescript".to_string(),
            Language::CSharp => "csharp".to_string(),
            Language::C => "c".to_string(),
            Language::Go => "golang".to_string(),
            Language::Kotlin => "kotlin".to_string(),
            Language::Swift => "swift".to_string(),
            Language::Rust => "rust".to_string(),
            Language::Ruby => "ruby".to_string(),
            Language::Php => "php".to_string(),
            Language::Dart => "dart".to_string(),
            Language::Scala => "scala".to_string(),
            Language::Elixir => "elixir".to_string(),
            Language::Erlang => "erlang".to_string(),
            Language::Racket => "racket".to_string(),
            Language::Cangjie => "cangjie".to_string(),
            Language::Bash => "bash".to_string(),
            Language::React => "react".to_string(),
            Language::MySQL => "mysql".to_string(),
            Language::MSSQL => "mssql".to_string(),
            Language::PostgreSQL => "postgresql".to_string(),
            Language::OracleSQL => "oraclesql".to_string(),
            Language::Pandas => "pythondata".to_string(),
        }
    }

    pub fn file_extension(&self) -> String {
        match self {
            Language::Cpp => "cpp".to_string(),
            Language::Java => "java".to_string(),
            Language::Python3 | Language::Python | Language::Pandas => "py".to_string(),
            Language::JavaScript => "js".to_string(),
            Language::TypeScript => "ts".to_string(),
            Language::CSharp => "cs".to_string(),
            Language::C => "c".to_string(),
            Language::Go => "go".to_string(),
            Language::Kotlin => "kt".to_string(),
            Language::Swift => "swift".to_string(),
            Language::Rust => "rs".to_string(),
            Language::Ruby => "rb".to_string(),
            Language::Php => "php".to_string(),
            Language::Dart => "dart".to_string(),
            Language::Scala => "scala".to_string(),
            Language::Elixir => "ex".to_string(),
            Language::Erlang => "erl".to_string(),
            Language::Racket => "rkt".to_string(),
            Language::Cangjie => "cj".to_string(),
            Language::Bash => "sh".to_string(),
            Language::React => "jsx".to_string(),
            Language::MySQL | Language::MSSQL | Language::PostgreSQL | Language::OracleSQL => {
                "sql".to_string()
            }
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::JavaScript => write!(f, "JavaScript"),
            Language::Cpp => write!(f, "C++"),
            Language::Java => write!(f, "Java"),
            Language::Python3 => write!(f, "Python3"),
            Language::Python => write!(f, "Python"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::CSharp => write!(f, "C#"),
            Language::C => write!(f, "C"),
            Language::Go => write!(f, "Go"),
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Swift => write!(f, "Swift"),
            Language::Rust => write!(f, "Rust"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Php => write!(f, "PHP"),
            Language::Dart => write!(f, "Dart"),
            Language::Scala => write!(f, "Scala"),
            Language::Elixir => write!(f, "Elixir"),
            Language::Erlang => write!(f, "Erlang"),
            Language::Racket => write!(f, "Racket"),
            Language::Cangjie => write!(f, "Cangjie"),
            Language::Bash => write!(f, "Bash"),
            Language::React => write!(f, "React"),
            Language::MySQL => write!(f, "MySQL"),
            Language::MSSQL => write!(f, "MSSQL"),
            Language::PostgreSQL => write!(f, "PostgreSQL"),
            Language::OracleSQL => write!(f, "OracleSQL"),
            Language::Pandas => write!(f, "Pandas"),
        }
    }
}
