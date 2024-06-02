use crate::dialogs::about_dialog::ABOUT_DIALOG;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(ABOUT_DIALOG).await?;

    Ok(())
}
