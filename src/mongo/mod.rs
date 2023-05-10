use std::error::Error;

extern crate dotenv_codegen;

use bson::Document;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Database,
};

pub struct Mongo {
    db: Database,
    client: Client,
}

impl Mongo {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client_uri: &str = dotenv!("MONGODB_URI");

        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;

        print!("mongo connected!");

        let client = Client::with_options(options)?;
        let db = client.database("lapis");

        Ok(Self { db, client })
    }

    pub fn get_collection(&self, collection: &str) -> Collection<Document> {
        self.db.collection(collection)
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_db(&self) -> &Database {
        &self.db
    }
}
