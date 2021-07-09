#[allow(unused)]
use dotenv::{dotenv, vars, var};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub host: String,
    pub port: i32
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            db_url: var("DATABASE_URL").unwrap(),
            host: var("HOST").unwrap(),
            port: var("PORT").unwrap().parse::<i32>().unwrap(),
        }
    }
}
