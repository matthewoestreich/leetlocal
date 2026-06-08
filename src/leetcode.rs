use crate::error::Error;
use reqwest::header::{CONTENT_TYPE, REFERER, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;

pub async fn get_question(title: &str) -> Result<Question, Error> {
    let query_str = r#"
        query questionData($titleSlug: String!) {
          question(titleSlug: $titleSlug) {
            title
            content
            exampleTestcases
          }
        }
    "#;

    let payload = serde_json::json!({
        "query": query_str,
        "variables": {
            "titleSlug": title,
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

#[derive(Deserialize, Debug, Default)]
pub struct Question {
    #[serde(rename = "title")]
    pub problem: String,
    pub content: String,
    #[serde(rename = "exampleTestcases")]
    pub example_testcases: String,
}

impl TryFrom<Value> for Question {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut question: Question = serde_json::from_value(value)?;
        question.content = html2md::parse_html(&question.content);
        Ok(question)
    }
}
