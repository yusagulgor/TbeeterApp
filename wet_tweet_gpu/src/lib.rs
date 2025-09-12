// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub mod gpu;
pub mod tweeter;

use mongodb::{Client, options::ClientOptions, Database};
use std::env;
use dotenvy::dotenv;

pub async fn get_db() -> Database {
    dotenv().ok();

    let uri = env::var("CLIENT_URI")
        .expect("CLIENT_URI environment variable is not set in .env");

    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "wet_twitter_clone".to_string());

    let options = ClientOptions::parse(&uri)
        .await
        .expect("Failed to parse MongoDB URI");

    let client = Client::with_options(options)
        .expect("Failed to initialize MongoDB client");

    client.database(&db_name)
}
