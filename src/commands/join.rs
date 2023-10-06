use songbird::join;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let mut sessions_lock = ctx.data().sessions.lock().await;
    let mut session = sessions_lock
        .get_mut(
            &ctx.guild_id()
                .expect("Command was not executed in a guild!"),
        )
        .expect("Could not create session");
    
    let songbird_mutex = songbird::get(ctx.serenity_context()).await.expect("Could not lock songbird");

    let songbird = songbird_mutex.clone();


    let channel_id = ctx.guild().unwrap()
        .voice_states.get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id).unwrap();

    let _handler = songbird.join(ctx.guild_id().unwrap(), channel_id).await;


    ctx.say("I have joined your voice channel").await?;
    Ok(())
}
