use anyhow::{Error, anyhow};
use poise::serenity_prelude::Guild;

use crate::Context;

use super::queue::Queue;

pub struct Session {
    queue: Queue,
    pub guild: Guild,
    paused: bool,
}

pub async fn create_session<'a>(context: &'a Context<'a>) -> Result<(), Error> {

    let session = Session {
        queue: Queue::default(),
        guild: context.guild().ok_or(anyhow!("Guild does not exist"))?,
        paused: false
    };

    let mut sessions = context.data().sessions.lock().await;

    if sessions.contains_key(&context.guild_id().expect("No guild")) {
        return Ok(())
    }

    sessions.insert(context.guild_id().expect("No guild"), session);

    Ok(())

}

impl Session {
}
