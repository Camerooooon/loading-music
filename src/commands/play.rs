use core::time;
use std::thread;

use crate::{
    music::{
        session::{self, Session},
        song::{find_youtube_song, Song},
    },
    Context, Error,
};

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "A song to look up"] query: String,
) -> Result<(), Error> {
    // Send the message
    let reply = ctx
        .send(|m| {
            m.embed(|e| {
                e.description(format!(":mag_right: **Searching** for *{}*", query))
                    .color(0x3498db)
            })
        })
        .await?;

    thread::sleep(time::Duration::from_millis(1000));

    session::create_session(&ctx).await?;
    let mut sessions_lock = ctx.data().sessions.lock().await;
    let session = sessions_lock
        .get_mut(
            &ctx.guild_id()
                .expect("Command was not executed in a guild!"),
        )
        .expect("Could not create session");

    let songs = find_youtube_song(query.clone(), &ctx.data().youtube).await;

    if songs.len() == 0 {
        reply
            .edit(ctx, |m| {
                m.embed(|e| {
                    e.description(format!(
                        ":skull: **Failed** no results returned for *{}*",
                        query
                    ))
                    .color(0xc0392b)
                })
            })
            .await?;
        return Ok(());
    }

    let chosen_song = songs[0].clone();

    session.queue.add_to_queue(
        chosen_song.clone(),
        ctx.author_member().await.expect("No author").user.clone(),
    );

    reply
        .edit(ctx, |m| {
            m.embed(|e| {
                e.description(format!(
                    ":white_check_mark: Adding **{}** to the queue",
                    chosen_song.meta.title
                ))
                .color(0x3498db)
            })
        })
        .await?;

    session.start_playback(&ctx).await;

    Ok(())
}
