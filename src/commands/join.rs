use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let mut sessions_lock = ctx.data().sessions.lock().await;
    let session = sessions_lock
        .get_mut(
            &ctx.guild_id()
                .expect("Command was not executed in a guild!"),
        )
        .expect("Could not create session");
    

    session.join_voice(&ctx).await;

    ctx.say("I have joined your voice channel").await?;
    Ok(())
}
