use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl Blog {
    pub fn new(title: Option<String>, description: Option<String>) -> Self {
        Self {
            id: None,
            title,
            description,
        }
    }
}

crud!(Blog {}, "blog");

pub fn connect_db() {
    let db_username = "pig";
    let db_password = "123456";
    let db_host = "127.0.0.1";
    let db_port = 3306;
    let db_schema = "example";
    let db_url = format!("mysql://{}:{}@{}:{}/{}", db_username, db_password, db_host, db_port, db_schema);

    match RB.init(MysqlDriver {}, &db_url) {
        Ok(()) => {
            println!("Database is connecting...");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

