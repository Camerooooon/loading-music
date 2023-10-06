use google_youtube3::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector, YouTube};
use poise::serenity_prelude::User;

#[derive(Clone)]
pub struct Song {
    pub source_id: String,
    pub meta: Meta,
}

#[derive(Clone)]
pub struct Meta {
    pub author: String,
    pub title: String,
}

#[derive(Clone)]
pub struct SongRequest {
    pub queued_by: User,
    pub song: Song,
}

pub async fn find_youtube_song(
    query: String,
    youtube: &YouTube<HttpsConnector<HttpConnector>>,
) -> Vec<Song> {
    let items = youtube
        .search()
        .list(&vec!["snippet".to_string()])
        .param("snippet", "true")
        .q(&query)
        .doit()
        .await
        .expect("Could not search")
        .1
        .items
        .unwrap_or(vec![]);
    let mut songs: Vec<Song> = vec![];

    for item in items {
        let id = item
            .clone()
            .id
            .expect("Youtube API returned bad data")
            .video_id
            .expect("Youtube API returned bad data");
        let author = item
            .clone()
            .snippet
            .expect("Youtube API returned bad data")
            .channel_title
            .unwrap_or("No author".to_string());
        let title = item
            .snippet
            .expect("Youtube API returned bad data")
            .title
            .unwrap_or("No title".to_string());
        let song: Song = Song {
            source_id: id.clone(),
            meta: Meta {
                author: author.clone(),
                title: title.clone(),
            },
        };

        songs.push(song);
    }

    return songs;
}
