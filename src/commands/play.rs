
use core::time;
use std::thread;

use poise::CreateReply;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "A song to look up"] query: String,
) -> Result<(), Error> {
    // Send the message
    let reply = ctx.send(|m| m.embed(|e| e.description(format!(":mag_right: **Searching** for *{}*", query)).color(0x3498db))).await?;

    thread::sleep(time::Duration::from_millis(1000));

    reply.edit(ctx, |m| m.embed(|e| e.description(format!(":skull: **Failed** no results returned for *{}*", query)).color(0xc0392b))).await?;
    Ok(())
}
