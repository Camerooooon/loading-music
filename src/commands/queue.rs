
use crate::{Context, Error, music::song::SongRequest};

#[poise::command(slash_command, prefix_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let sessions_lock = ctx.data().sessions.lock().await;
    let session = sessions_lock
        .get(
            &ctx.guild_id()
                .expect("Command was not executed in a guild!"),
        )
        .expect("Could not create session");


    ctx
        .send(|m| {
            m.embed(|e| {
                e.description(format!("{}", create_queue_message(&session.queue.requests)))
                    .color(0x3498db)
            })
        })
        .await?;
    Ok(())
}

fn create_queue_message(requests: &Vec<SongRequest>) -> String {

    let mut song_list = "".to_string();

    if requests.len() == 0 {
        song_list = "There is nothing in the queue".to_string();
    } else {

        let mut i = 0;
        for request in requests {
            if i == 0 {
                song_list = song_list + "**Now Playing:** " + &request.song.meta.title + "\n";
            } else {
                song_list = song_list + "**" + &i.to_string() + ".** " + &request.song.meta.title + "\n";
            }
            i = i + 1;
        }

    }

    song_list

}
