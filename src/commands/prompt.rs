use poise::serenity_prelude::Http;

use crate::utils::replicate::run_prompt;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn prompt(ctx: Context<'_>, prompt: Option<String>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        panic!("You are not in a guild!")
    };
    ctx.defer().await?;
    let guild = Http::get_guild_with_counts(ctx.http(), guild_id).await?;
    let verified_prompt = prompt.expect("Prompt not found");
    let response = run_prompt(&verified_prompt, &guild.name).await.unwrap();
    ctx.say(response).await?;
    Ok(())
}
