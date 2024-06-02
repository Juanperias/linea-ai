mod commands;
mod dialogs;
mod env;
mod queries;
mod utils;

use commands::about::about;
use commands::chat::chat;
use commands::prompt::prompt;
use commands::set_model::set_model;
use dotenv::dotenv;
use env::get_env::get_env;
use std::vec;
use utils::db;

use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    db::init_db()
        .await
        .expect("Database could not be initialized");
    println!("db inizialized 🗼");

    let discord_token = get_env().discord_token;
    let options = poise::FrameworkOptions {
        commands: vec![about(), set_model(), prompt(), chat()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("Linea.".into()),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    println!("The bot is running 🤖");

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
