use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx_mysql::{MySqlPool};
use warp::reject::Reject;
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

pub async fn get_all_songs(pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_all_songs = sqlx::query_as!(
        Song,
        "SELECT
            UID as uid,
            SONG_NAME as song_name,
            PRIMARY_ARTIST_ID as primary_artist_id,
            URL as url
        FROM rust_api.SONGS"
    )
        .fetch_all(&pool)
        .await;
    let ok_response = query_all_songs.unwrap();
    Ok(warp::reply::json(&ok_response))
}