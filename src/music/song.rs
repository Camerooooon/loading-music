use poise::serenity_prelude::User;

pub struct Song {
    queued_by: User,
    yt_source_url: String
}