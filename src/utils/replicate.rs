use std::fmt::Error;

use crate::{
    db::{add_prompt_to_user_context, get_user_context, search_server_model, Conversation},
    env::get_env::get_env,
};
use replicate_rust::{config::Config, Replicate};
use tokio::task;

pub async fn run_prompt(prompt: &str, server_name: &String) -> Result<String, Error> {
    let replicate_api_key = get_env().replicate_api_key;
    let config = Config {
        auth: replicate_api_key.clone(),
        ..Default::default()
    };

    let replicate = Replicate::new(config);
    let server_search = search_server_model(server_name).await;
    let server = match server_search {
        Ok(mut servers) => servers.pop(),
        Err(_) => None,
    }
    .unwrap();

    let prompt_cloned = prompt.to_string();

    let mut inputs = std::collections::HashMap::new();
    inputs.insert("prompt", prompt_cloned);
    let version = server.model;

    let response = task::spawn_blocking(move || replicate.run(version.as_str(), inputs))
        .await
        .expect("REPLICATE NOT FOUND")
        .unwrap();
    let ai_response = response.output.clone().unwrap();

    let mut text = String::new();

    if let Some(arr) = response.output.map(|_arr| ai_response.as_array()) {
        text = arr
            .map(|v| v.iter().map(|v| v.as_str().unwrap()).collect::<String>())
            .unwrap();
    }

    Ok(text)
}

pub async fn chat_with_model(
    user_id: String,
    prompt: &str,
    server_name: &String,
) -> Result<String, Error> {
    let context_vec = get_user_context(user_id.clone()).await;
    let context = match context_vec {
        Ok(mut contexts) => contexts.pop(),
        Err(_) => None,
    }
    .unwrap();

    println!("{:?}", context);
    let message = format!("For a better conversation you will be provided with a context which you must follow where Human will be the user and AI will be you. {} End of context Human: {}", context.chat, prompt);

    let response = run_prompt(&message, server_name)
        .await
        .expect("An error occurred while the model was running");
    add_prompt_to_user_context(Conversation {
        user_id,
        human_content: prompt.to_string(),
        ai_content: response.clone(),
    })
    .await
    .expect("ERROR SAVING THE CONTEXT");

    Ok(response)
}
