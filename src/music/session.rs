use anyhow::{anyhow, Error};
use poise::serenity_prelude::Guild;

use crate::Context;

use super::queue::Queue;

#[derive(Clone)]
pub struct Session {
    pub queue: Queue,
    pub guild: Guild,
    pub paused: bool,
}

pub async fn create_session<'a>(context: &'a Context<'a>) -> Result<(), Error> {
    let session = Session {
        queue: Queue::default(),
        guild: context.guild().ok_or(anyhow!("Guild does not exist"))?,
        paused: false,
    };

    let mut sessions = context.data().sessions.lock().await;

    if sessions.contains_key(&context.guild_id().expect("No guild")) {
        return Ok(());
    }

    sessions.insert(context.guild_id().expect("No guild"), session);

    Ok(())
}

impl Session {
    pub async fn join_voice<'a>(&mut self, ctx: &'a Context<'a>) {

        let songbird_mutex = songbird::get(ctx.serenity_context()).await.expect("Could not lock songbird");

        let songbird = songbird_mutex.clone();


        let channel_id = ctx.guild().unwrap()
            .voice_states.get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id).unwrap();

        let _handler = songbird.join(ctx.guild_id().unwrap(), channel_id).await;
    }

    pub async fn start_playback<'a>(&mut self, ctx: &'a Context<'a>) {
        let manager = songbird::get(ctx.serenity_context()).await
            .unwrap().clone();

        let handler_lock = manager.get(ctx.guild_id().unwrap()).unwrap();

        let handler = handler_lock.lock();


        let song = match self.queue.requests.get(0) {
            Some(s) => {s},
            None => {return},
        };
        
        let source = songbird::ytdl(&song.song.source_id).await.unwrap();

        handler.await.play_source(source);
    }
}
