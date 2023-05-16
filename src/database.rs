use anyhow::Result;
use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

// Define a lazy static database connection
lazy_static! {
    static ref DATABASE_CLIENT: Client = {
        tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(async {
                Client::with_uri_str(dotenv!("MONGODB_URI"))
                    .await
                    .expect("Failed to connect to the database")
            })
    };
}

// Get a collection from the database
pub fn get_collection(collection_name: &str) -> Collection<Document> {
    let db = DATABASE_CLIENT.database("lapis");
    let collection = db.collection(collection_name);
    collection
}

// Insert a document into a collection
pub fn insert_document(collection: &Collection<Document>, document: Document) -> Result<()> {
    tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(async {
            collection
                .insert_one(document, None)
                .await
                .map(|_| ())
                .map_err(Into::into)
        })
}
