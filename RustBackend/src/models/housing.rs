use std::fmt;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx_mysql::{MySqlPool};
use warp::reject::Reject;

#[derive(Debug)]
pub struct InvalidPost;
#[derive(Debug)]
pub struct GetError;
impl Reject for GetError {}
impl Reject for InvalidPost {}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HousingLocation {
    pub id: u64,
    pub name: String,
    pub city: String,
    pub state: String,
    pub photo: String,
    pub available_units: u64,
    pub wifi: i8,
    pub laundry: i8
}

impl HousingLocation {
    pub fn new(
        name: String,
        city: String,
        state: String,
        photo: String,
        available_units: u64,
        wifi: i8,
        laundry: i8

    ) -> Self {
        HousingLocation {
            id: 0,
            name,
            city,
            state,
            photo,
            available_units,
            wifi,
            laundry
        }
    }

    pub async fn insert_housing_location(pool: sqlx::MySqlPool, hl: HousingLocation) -> u64 {
        let query_insert_hl = sqlx::query(
            "INSERT INTO rust_api.HOUSING_LOCATION (NAME, CITY, STATE, PHOTO, AVAILABLE_UNITS, WIFI, LAUNDRY)\
                VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(hl.name)
            .bind(hl.city)
            .bind(hl.state)
            .bind(hl.photo)
            .bind(hl.available_units)
            .bind(hl.wifi)
            .bind(hl.laundry)
            .execute(&pool)
            .await;

        let res = match query_insert_hl {
            Ok(r) => r.last_insert_id(),
            Err(e) => 0
        };

        res
    }

    pub async fn get_housing_location_by_id(pool: sqlx::MySqlPool, id: u64)
                                            -> Result<HousingLocation, Error> {
        let query = sqlx::query_as!(
        HousingLocation,
        "SELECT \
            ID AS id,\
            NAME AS name,\
            CITY AS city,\
            STATE AS state,\
            PHOTO AS photo,\
            AVAILABLE_UNITS AS available_units,\
            WIFI AS wifi,\
            LAUNDRY AS laundry \
        FROM rust_api.HOUSING_LOCATION WHERE ID = ?",
        id
    )
            .fetch_one(&pool)
            .await;

        query
    }

    pub async fn get_all_housing_locations(pool: sqlx::MySqlPool)
        -> Result<Vec<HousingLocation>, Error> {
        let query = sqlx::query_as!(
            HousingLocation,
            "SELECT \
                ID AS id,\
                NAME AS name,\
                CITY AS city,\
                STATE AS state,\
                PHOTO AS photo,\
                AVAILABLE_UNITS AS available_units,\
                WIFI AS wifi,\
                LAUNDRY AS laundry \
            FROM rust_api.HOUSING_LOCATION"
        )
            .fetch_all(&pool)
            .await;

        query

    }
}

