use std::fs;
use warp::Filter;
use crate::mariadb::*;
use sqlx_mysql::{MySql, MySqlPool, MySqlQueryResult};
use sqlx::{Pool, Error};
use warp::http::StatusCode;
use crate::models;
use models::{people::*, music_models::*, housing::*};


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



pub async fn post_housing_location(pool: sqlx::MySqlPool, hl: HousingLocation)
    -> Result<impl warp::Reply, warp::Rejection> {
    let hl_insert = HousingLocation::insert_housing_location(pool.clone(), hl).await;

    if hl_insert < 1 {
        Err(warp::reject::custom(InvalidPost))
    } else {
        let hl = HousingLocation::get_housing_location_by_id(pool.clone(), hl_insert).await;
        println!("{:#?}", hl);
        match hl {
            Ok(o) => Ok(warp::reply::json(&o)),
            Err(e) => Err(warp::reject::custom(GetError))
        }
    }

}

pub async fn handle_housing_location_rejection(err: warp::Rejection)
    -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(_) = err.find::<InvalidPost>() {
        Ok(warp::reply::with_status("UNPROCESSABLE_CONTENT", StatusCode::UNPROCESSABLE_ENTITY))
    } else {
        eprintln!("unhandled error: {:#?}", err);
        Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

// can delete below this line
pub async fn another_thread() -> Result<impl warp::Reply, warp::Rejection> {
    let contents: String = fs::read_to_string("www/hi.html").unwrap();

    Ok(warp::reply::html(contents))
}

pub async fn handle_get_all_houses(pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    let v = HousingLocation::get_all_housing_locations(pool).await;
    match v {
        Ok(v) => Ok(warp::reply::json(&v)),
        Err(e) => Err(warp::reject::custom(GetError))
    }
}