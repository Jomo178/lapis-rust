use anyhow::Result;
use dotenv_codegen::dotenv;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    results::InsertOneResult,
    Client, Collection, Database,
};
use poise::futures_util::TryStreamExt;
use std::borrow::Borrow;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DATABASE: Mutex<Option<Database>> = Mutex::new(None);
}

pub async fn connect() -> Result<String> {
    let client = Client::with_uri_str(dotenv!("MONGODB_URI")).await?;
    let mut db_lock = DATABASE.lock().expect("Failed to acquire lock");
    *db_lock = Some(client.database("lapis"));
    Ok("Mongo Connected".to_string())
}

pub fn get_collection(collection_name: &str) -> Collection<Document> {
    let db_lock = DATABASE.lock().expect("Failed to acquire lock");
    let db = db_lock
        .as_ref()
        .expect("Database connection not established");
    db.collection(collection_name)
}

pub async fn find(
    database: Collection<Document>,
    filter: impl Into<Option<Document>>,
) -> Result<Option<Document>> {
    let result = database
        .find(filter, FindOptions::default())
        .await
        .unwrap()
        .try_next()
        .await?;

    Ok(result)
}

pub async fn findOne(
    database: Collection<Document>,
    filter: impl Into<Option<Document>>,
) -> Result<Option<Document>> {
    let result = database.find_one(filter, None).await?;

    Ok(result)
}

pub async fn create(database: Collection<Document>, document: Document) -> Result<Document> {
    database.insert_one(document.clone(), None).await?;

    Ok(document)
}
