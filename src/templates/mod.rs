pub mod javascript;

pub trait QuestionTemplate {
    fn new_template(question_name: &str, question_description: &str) -> String;
}
