use std::fs;
use std::future::Future;
use tokio::{task,
            time::{sleep, Duration}
};
use crate::handlers::*;
use crate::mariadb::MariaDbConn;
use warp::{Filter};


use models::{
    people::*,
    users::*,
    music_models::*
};

mod handlers;
mod routes;
mod mariadb;

mod models {
    pub mod music_models;
    pub mod people;
    pub mod users;
    pub mod housing;
}

#[tokio::main]
async fn main() {
    std::env::var("DATABASE_URL").expect("Requires database url");

    println!("Hello, world!");
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "PUT"]);

    let mut mdb = MariaDbConn::new(String::from("mariadb://root:my-secret-pw@127.0.0.1:3306/rust_api"));
    mdb.connect().await;

    let mdb_option = mdb.pool.unwrap();
    let pool_filter = warp::any().map(move || mdb_option.clone());

    // can delete routes below. this is for experimentation
    let silly = warp::path("silly")
        .and(warp::get())
        .and_then(silly_handler);

    let new_song = warp::path!("songs" / "new_song")
        .and(warp::post())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(add_song_handler);

    let new_house = warp::path!("housing" / "new_house")
        .and(warp::post())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(post_housing_location);

    let get_all_houses = warp::path!("housing" / "houses")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handle_get_all_houses);

    let get_house_by_id = warp::path!("housing" / "house"/ u64)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handle_get_house_by_id);

    // stop deleting routes
    let people = warp::path("people")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(get_all_people);

    let songs = warp::path("songs")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(get_all_songs);

    let routes = people
        .or(songs)
        .or(silly)
        .or(new_song)
        .or(new_house)
        .or(get_all_houses)
        .or(get_house_by_id)
        .with(cors);


    // can delete this block
    let h = task::spawn(async {
        another_warp_server().await;
    });
    // don't delete anymore


    println!("Server started at http://127.0.0.1:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}


// Can delete below this. Just testing out tokio spawning additional threads
async fn silly_fn() -> String {
    println!("PRINT");
    sleep(Duration::from_secs(20)).await;
    String::from("Waited 20 secs")

}

async fn silly_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let h = task::spawn(async {
        silly_fn()
    });
    let result = h.await.unwrap().await;
    let p = Person::new(result, Colors::blue);
    Ok(warp::reply::json(&p))
}

async fn another_warp_server() {
    let new_thread_path = warp::path::end()
        .and_then(another_thread);
    println!("Server started at http://127.0.0.1:8001");
    warp::serve(new_thread_path).run(([127,0,0,1], 8001)).await;
}

async fn add_song(pool: sqlx::MySqlPool, song: Song) ->  u64 {
    let insert_res = sqlx::query(
        "INSERT INTO rust_api.SONGS \
            (song_name, primary_artist_id, url)\
            VALUES (?, ?, ?)"
    )
        .bind(song.song_name.clone())
        .bind(song.primary_artist_id.unwrap_or_else(|| 0))
        .bind(song.url.clone().unwrap_or_else(|| "".to_string()))
        .execute(&pool)
        .await;

    let res_id = match insert_res {
        Ok(r) => {
            let s = r.last_insert_id();
            println!("{:#?}", s);
            s
        },
        Err(_e) => 0
    };

    res_id

}

async fn add_song_handler(pool: sqlx::MySqlPool, song: Song) -> Result<impl warp::Reply, warp::Rejection> {
    let id = add_song(pool, song).await;
    println!("ID: {id}");
    let html = fs::read_to_string("www/hi.html").unwrap();
    Ok(warp::reply::html(html))
}