use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub async fn get_database() -> Result<mongodb::Database, Box<dyn Error>>{
    let mongodb_uri = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set.")?;
    if mongodb_uri.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "DATABASE_URL cannot be empty")));
    }

    let database_name = std::env::var("MONGO_INITDB_DATABASE").map_err(|_| "MONGO_INITDB_DATABASE must be set.")?;
    if database_name.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "MONGO_INITDB_DATABASE cannot be empty")));
    }

    let mut client_options = ClientOptions::parse(&mongodb_uri).await?;
    client_options.app_name = Some(database_name.clone());

    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);

    Ok(database)
}