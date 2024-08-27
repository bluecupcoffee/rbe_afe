use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Song {
    pub uid: u64,
    pub song_name: String,
    pub primary_artist_id: Option<u64>,
    pub url: Option<String>
}

impl Song {
    pub fn new(name: String, artist_id: Option<u64>, url: Option<String>) -> Song {
        Song {
            uid: 0,
            song_name: name,
            primary_artist_id: artist_id,
            url
        }
    }
}
