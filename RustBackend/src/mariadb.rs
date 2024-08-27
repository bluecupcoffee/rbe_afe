use sqlx::{Pool, Error};
use sqlx_mysql::{MySql, MySqlPool};


pub struct MariaDbConn {
    pub pool: Option<MySqlPool>,
    url: String
}

impl MariaDbConn {
    pub fn new(url: String) -> MariaDbConn {
        MariaDbConn {
            pool: None,
            url
        }
    }
    pub async fn connect(&mut self) -> () {
        let mp = MySqlPool::connect(self.url.as_str()).await;
        self.pool = Some(mp.unwrap())

    }
}

async fn connect() -> Result<MySqlPool, Error> {
    return MySqlPool::connect("mariadb://root:my-secret-pw@127.0.0.1:3306/rust_api").await;
}

#[tokio::test]
async fn test_connect() {
    let res = connect().await;
    match res {
        Ok(p) => {
            println!("{:#?}", p);
            assert!(true)
        },
        Err(e) => {
            eprintln!("{:#?}", e);
            assert!(false);
        }
    }
}