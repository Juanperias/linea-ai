use crate::queries::{
    add_prompt_to_user_context::ADD_PROMPT_TO_USER_CONTEXT_QUERY,
    create_server_query::CREATE_SERVER_QUERY, create_server_table::CREATE_SERVER_TABLE_QUERY,
    create_user_context_query::CREATE_USER_CONTEXT_QUERY,
    create_user_context_table::CREATE_USER_CONTEXT_TABLE_QUERY,
    delete_context_query::DELETE_CONTEXT_QUERY, get_user_context_query::GET_USER_CONTEXT_QUERY,
    search_server_model::SEARCH_SERVER_MODEL_QUERY,
};
use core::panic;
use once_cell::sync::Lazy;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::sync::Arc;

static POOL: Lazy<Arc<SqlitePool>> = Lazy::new(|| {
    Arc::new(
        SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:")
            .unwrap(),
    )
});

#[derive(Clone, sqlx::FromRow, Debug)]
pub struct Server {
    pub server_name: String,
    pub model: String,
}
#[derive(Clone, sqlx::FromRow, Debug)]
pub struct UserContext {
    pub user_id: String,
    pub chat: String,
}

#[derive(Clone, Debug)]
pub struct Conversation {
    pub user_id: String,
    pub human_content: String,
    pub ai_content: String,
}

pub async fn init_db() -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_SERVER_TABLE_QUERY)
        .execute(&**POOL)
        .await?;
    sqlx::query(CREATE_USER_CONTEXT_TABLE_QUERY)
        .execute(&**POOL)
        .await?;

    Ok(())
}

pub async fn create_server(server: Server) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_SERVER_QUERY)
        .bind(&server.server_name)
        .bind(&server.model)
        .execute(&**POOL)
        .await?;

    Ok(())
}

pub async fn search_server_model(server_name: &String) -> Result<Vec<Server>, sqlx::Error> {
    let servers = sqlx::query_as::<_, Server>(SEARCH_SERVER_MODEL_QUERY)
        .bind(server_name)
        .fetch_all(&**POOL)
        .await?;

    Ok(servers)
}

pub async fn get_user_context(user_id: String) -> Result<Vec<UserContext>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserContext>(GET_USER_CONTEXT_QUERY)
        .bind(&user_id)
        .fetch_all(&**POOL)
        .await?;

    if users.is_empty() {
        create_user_context(UserContext {
            user_id,
            chat: "".to_string(),
        })
        .await?;

        panic!("User does not have context");
    }

    Ok(users)
}

pub async fn create_user_context(user_context: UserContext) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_USER_CONTEXT_QUERY)
        .bind(&user_context.user_id)
        .bind(&user_context.chat)
        .execute(&**POOL)
        .await?;

    Ok(())
}

pub async fn add_prompt_to_user_context(conversation: Conversation) -> Result<(), sqlx::Error> {
    let context = format!(
        "HUMAN: {} AI: {}",
        conversation.human_content, conversation.ai_content
    );

    sqlx::query(ADD_PROMPT_TO_USER_CONTEXT_QUERY)
        .bind(&context)
        .bind(&conversation.user_id)
        .execute(&**POOL)
        .await?;

    Ok(())
}

pub async fn delete_user_context(user_id: String) -> Result<(), sqlx::Error> {
    sqlx::query(DELETE_CONTEXT_QUERY)
        .bind(&user_id)
        .execute(&**POOL)
        .await?;

    Ok(())
}
