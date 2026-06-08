use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GraphQLPayload {
    pub query: String,
    pub variables: Variables,
}

#[derive(Serialize)]
pub struct Variables {
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub question: Option<Question>,
}

#[derive(Deserialize, Debug)]
pub struct Question {
    pub title: String,
    pub content: String,
    #[serde(rename = "exampleTestcases")]
    pub example_testcases: String,
}
