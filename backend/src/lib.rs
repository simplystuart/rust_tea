use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Message {
    pub text: String,
}
