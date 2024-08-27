use std::future::Future;
use tokio::{task,
            time::{sleep, Duration}
};
use crate::handlers::*;
use crate::mariadb::MariaDbConn;
use warp::{Filter};


use models::people::*;

mod handlers;
mod routes;
mod mariadb;

mod models {
    pub mod music_models;
    pub mod people;
}

#[tokio::main]
async fn main() {

    println!("Hello, world!");
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"]);

    let p = Person::new(String::from("hi there"), Colors::blue);




    let mut mdb = MariaDbConn::new(String::from("mariadb://root:my-secret-pw@127.0.0.1:3306/rust_api"));
    mdb.connect().await;

    let mdb_option = mdb.pool.unwrap();
    let pool_filter = warp::any().map(move || mdb_option.clone());

    let silly = warp::path("silly")
        .and(warp::get())
        .and_then(silly_handler);

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