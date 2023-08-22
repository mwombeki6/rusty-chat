use serde:: Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub message: Vec<Message>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}