use super::song::Song;

pub struct Queue {
    pub songs: Vec<Song>
}

impl Queue {
    fn add_to_queue(&mut self, song: Song) {
        self.songs.push(song);
    }

    pub(crate) fn default() -> Queue {
        Queue { songs: vec![] }
    }
}
