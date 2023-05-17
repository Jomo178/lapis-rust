use dotenv_codegen::dotenv;
use mongodb::{
    bson::{doc, Document},
    Client, Database,
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DATABASE: Mutex<Option<Database>> = Mutex::new(None);
}

pub async fn connect() -> Result<String, String> {
    match Client::with_uri_str(dotenv!("MONGODB_URI")).await {
        Ok(client) => {
            let mut db_lock = DATABASE.lock().expect("Failed to acquire lock");
            *db_lock = Some(client.database("lapis"));
            Ok("Mongo Connected".to_string())
        }
        Err(err) => Err(format!("Failed to connect to the database: {}", err)),
    }
}

pub fn get_collection(collection_name: &str) -> mongodb::Collection<Document> {
    let db_lock = DATABASE.lock().expect("Failed to acquire lock");
    let db = db_lock
        .as_ref()
        .expect("Database connection not established");
    db.collection(collection_name)
}
