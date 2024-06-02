use crate::dialogs::set_model_dialog::get_dialog;
use crate::utils::db::{create_server, Server};
use crate::{Context, Error};
use poise::serenity_prelude::Http;

#[poise::command(slash_command)]
pub async fn set_model(ctx: Context<'_>, model: Option<String>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        panic!("You are not in a guild!")
    };
    let guild = Http::get_guild_with_counts(ctx.http(), guild_id).await?;

    let server = Server {
        server_name: guild.name.clone(),
        model: model.unwrap(),
    };

    create_server(server).await?;

    ctx.say(get_dialog(guild.name.clone())).await?;

    Ok(())
}
