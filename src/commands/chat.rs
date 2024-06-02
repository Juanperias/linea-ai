use crate::{
    dialogs::chat_dialog::get_dialog, serenity::Http, utils::replicate::chat_with_model, Context,
    Error,
};

#[poise::command(slash_command)]
pub async fn chat(ctx: Context<'_>, prompt: Option<String>) -> Result<(), Error> {
    let verified_prompt = prompt.expect("Prompt not found");
    let Some(guild_id) = ctx.guild_id() else {
        panic!("You are not in a guild!")
    };
    let guild = Http::get_guild_with_counts(ctx.http(), guild_id).await?;
    let user_id = ctx.author().id.to_string();
    let username = ctx.author().name.clone();
    let server_name = guild.name;

    ctx.defer().await?;

    let response = chat_with_model(user_id, &verified_prompt, &server_name).await?;
    ctx.say(get_dialog(response, username)).await?;
    Ok(())
}
