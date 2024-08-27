use sqlx_mysql::MySqlPool;
use warp::Filter;
use super::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_song()
        .or(get_root())
}
fn get_song() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("songs" / u64)
        .and(warp::get())
        .and_then(handlers::get_song)
}

// fn get_all_people(pool: &MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("people")
//         .and(warp::get())
//         .and_then(handlers::get_all_people)
// }

fn get_root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!()
        .and(warp::get())
        .and_then(handlers::get_root)
}