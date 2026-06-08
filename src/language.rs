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
    Golang,
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
    MSSQL,
    PostgreSQL,
    OracleSQL,
    Pandas,
}

impl Language {
    pub fn slug(&self) -> String {
        match self {
            Language::JavaScript => "javascript".to_string(),
            Language::Cpp => "".to_string(),
            Language::Java => "".to_string(),
            Language::Python3 => "".to_string(),
            Language::Python => "".to_string(),
            Language::TypeScript => "".to_string(),
            Language::CSharp => "".to_string(),
            Language::C => "".to_string(),
            Language::Golang => "".to_string(),
            Language::Kotlin => "".to_string(),
            Language::Swift => "".to_string(),
            Language::Rust => "".to_string(),
            Language::Ruby => "".to_string(),
            Language::Php => "".to_string(),
            Language::Dart => "".to_string(),
            Language::Scala => "".to_string(),
            Language::Elixir => "".to_string(),
            Language::Erlang => "".to_string(),
            Language::Racket => "".to_string(),
            Language::Cangjie => "".to_string(),
            Language::Bash => "".to_string(),
            Language::React => "".to_string(),
            Language::MySQL => "".to_string(),
            Language::MSSQL => "".to_string(),
            Language::PostgreSQL => "".to_string(),
            Language::OracleSQL => "".to_string(),
            Language::Pandas => "".to_string(),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::JavaScript => write!(f, "JavaScript"),
            Language::Cpp => write!(f, ""),
            Language::Java => write!(f, ""),
            Language::Python3 => write!(f, ""),
            Language::Python => write!(f, ""),
            Language::TypeScript => write!(f, ""),
            Language::CSharp => write!(f, ""),
            Language::C => write!(f, ""),
            Language::Golang => write!(f, ""),
            Language::Kotlin => write!(f, ""),
            Language::Swift => write!(f, ""),
            Language::Rust => write!(f, ""),
            Language::Ruby => write!(f, ""),
            Language::Php => write!(f, ""),
            Language::Dart => write!(f, ""),
            Language::Scala => write!(f, ""),
            Language::Elixir => write!(f, ""),
            Language::Erlang => write!(f, ""),
            Language::Racket => write!(f, ""),
            Language::Cangjie => write!(f, ""),
            Language::Bash => write!(f, ""),
            Language::React => write!(f, ""),
            Language::MySQL => write!(f, ""),
            Language::MSSQL => write!(f, ""),
            Language::PostgreSQL => write!(f, ""),
            Language::OracleSQL => write!(f, ""),
            Language::Pandas => write!(f, ""),
        }
    }
}
