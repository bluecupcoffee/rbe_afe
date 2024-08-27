use std::fs;
use warp::Filter;
use crate::mariadb::*;
use sqlx_mysql::{MySql, MySqlPool, MySqlQueryResult};
use sqlx::{Pool, Error};

use crate::models;
use models::{people::*, music_models::*};


pub async fn get_song(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let song = Song {
        uid: 0,
        song_name: "".to_string(),
        primary_artist_id: None,
        url: None,
    };
    Ok(warp::reply::json(&song))
}

pub async fn get_root() -> Result<impl warp::Reply, warp::Rejection> {
    let html = fs::read_to_string("www/hi.html").unwrap();
    Ok(warp::reply::html(html))
}

pub async fn get_all_people(pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
        let query_all_people = sqlx::query_as!(
            Person,
            "SELECT * FROM rust_api.test_table"
        )
            .fetch_all(&pool)
            .await;
        let ok_response = query_all_people.unwrap();
        Ok(warp::reply::json(&ok_response))

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
// can delete below this line
pub async fn another_thread() -> Result<impl warp::Reply, warp::Rejection> {
    let contents: String = fs::read_to_string("www/hi.html").unwrap();

    Ok(warp::reply::html(contents))
}
