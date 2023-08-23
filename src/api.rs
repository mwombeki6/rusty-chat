use leptos::*;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    })
    .await.unwrap();
    
    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant";
    let mut history = format!(
        "{character_name}:Hello - How may I help you today?\n\
        {user_name}:What is the capital of Tanzania?\n\
        {character_name}:Dodoma is the capital of Tanzania.\n"
    );
}