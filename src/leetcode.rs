use crate::{Error, Language};
use reqwest::header::{CONTENT_TYPE, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn get_question(title_slug: &str) -> Result<Question, Error> {
    let query_str = r#"
        query questionData($titleSlug: String!) {
          question(titleSlug: $titleSlug) {
            title
            content
            exampleTestcaseList
            hints
            metaData
            codeSnippets {
              lang
              langSlug
              code
            }
            topicTags {
              name
              slug
            }
          }
        }
    "#;

    let payload = serde_json::json!({
        "query": query_str,
        "variables": {
            "titleSlug": title_slug,
        }
    });

    let client = reqwest::Client::new();

    let res: serde_json::Value = client
        .post("https://leetcode.com/graphql")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header(REFERER, "https://leetcode.com")
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    let raw_question = res
        .get("data")
        .and_then(|v| v.get("question"))
        .cloned()
        .ok_or(Error::QuestionNotFound)?;

    if raw_question == serde_json::Value::Null {
        return Err(Error::QuestionNotFound);
    }

    Question::try_from(raw_question)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Question {
    #[serde(rename = "title")]
    pub problem: String,
    #[serde(rename = "exampleTestcaseList")]
    pub example_testcase_list: Vec<String>,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
    #[serde(rename = "topicTags")]
    pub topics: Vec<Topic>,
    #[serde(rename = "metaData", deserialize_with = "from_json_string")]
    pub metadata: Metadata,
    pub content: String,
    pub hints: Vec<String>,
}

impl Question {
    pub fn get_snippet(&self, language: &Language) -> Option<String> {
        let lang_slug = language.slug();
        let snip = self
            .code_snippets
            .iter()
            .find(|cs| cs.lang_slug == lang_slug)
            .cloned()?;

        // Some languages need code prepended to the snippet.
        // For example, Rust only provides `impl Solution { /*...*/ }` and
        // still needs the `pub struct Solution` prepended.
        Some(match language {
            Language::Rust => format!("pub struct Solution;\n\n{}", snip.code),
            _ => snip.code,
        })
    }

    pub fn create_readme_data(&self) -> String {
        let mut readme_data = format!("# {}\n\n", self.problem);
        readme_data.push_str(&html2md::parse_html(&self.content));

        if !self.topics.is_empty() {
            readme_data.push_str(
                "\n\n---\n\n**Topics:**\n\n<details>\n\t<summary>Click to show topics</summary>\n",
            );
            for topic in &self.topics {
                readme_data.push_str(&format!("\n- {}", topic.name));
            }
            readme_data.push_str("\n\n</details></br>");
        }

        if !self.hints.is_empty() {
            readme_data.push_str(
                "\n\n---\n\n**Hints:**\n\n<details>\n\t<summary>Click to show hints</summary>\n",
            );
            for hint in &self.hints {
                readme_data.push_str(&format!("\n- {hint}"));
            }
            readme_data.push_str("\n\n</details></br>")
        }

        readme_data
    }
}

impl TryFrom<Value> for Question {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(serde_json::from_value::<Question>(value)?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Topic {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MetadataParam {
    pub name: String,
    pub r#type: String,
    pub dealloc: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MetadataReturn {
    pub r#type: String,
    pub dealloc: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Metadata {
    pub name: String,
    pub params: Vec<MetadataParam>,
    pub r#return: MetadataReturn,
}

fn from_json_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: serde::de::DeserializeOwned,
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}
