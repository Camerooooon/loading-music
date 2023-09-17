use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn sessions(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let sessions_lock = ctx.data().sessions.lock().await;
    let session = sessions_lock.get(&ctx.guild_id().expect("Command was not executed in a guild!")).expect("Could not create session");
    let response = format!("You are in session id: {}", session.guild.id);
    ctx.say(response).await?;
    Ok(())
}
