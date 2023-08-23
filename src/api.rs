use leptos::*;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
}