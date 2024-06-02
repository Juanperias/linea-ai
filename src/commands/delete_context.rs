use crate::{db::delete_user_context, dialogs::delete_dialog::DELETE_DIALOG, Context, Error};

#[poise::command(slash_command)]
pub async fn delete_context(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.to_string();

    delete_user_context(user_id).await?;

    ctx.say(DELETE_DIALOG).await?;
    Ok(())
}
