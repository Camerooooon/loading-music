use poise::serenity_prelude::User;

use super::song::{Song, SongRequest};

#[derive(Clone)]
pub struct Queue {
    pub requests: Vec<SongRequest>,
}

impl Queue {
    pub fn add_to_queue(&mut self, song: Song, queued_by: User) {
        self.requests.push(SongRequest { song, queued_by });
    }

    pub(crate) fn default() -> Queue {
        Queue { requests: vec![] }
    }
}
