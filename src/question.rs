use crate::{Language, error::Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Question {
    #[serde(rename = "title")]
    pub problem: String,
    #[serde(rename = "exampleTestcases")]
    pub example_testcases: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
    pub content: String,
    pub hints: Vec<String>,
}

impl Question {
    pub fn has_language(&self, language: &Language) -> bool {
        let lang_slug = language.slug();
        self.code_snippets
            .iter()
            .find(|cs| cs.lang_slug == lang_slug)
            .is_some()
    }
}

impl TryFrom<Value> for Question {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut question: Question = serde_json::from_value(value)?;
        question.content = html2md::parse_html(&question.content);
        Ok(question)
    }
}

// =====================
// ==== CodeSnippet ====
// =====================

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}
