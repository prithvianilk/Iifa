use std::env;

use mongodb::bson::{doc, Uuid};
use mongodb::results::UpdateResult;
use mongodb::{Client, Collection};
use mongodb::error::Error;

use crate::domain::decision_tree::DecisionTree;

const MONGODB_URI: &str = "MONGODB_URI";
const DB_NAME: &str = "whomping_willow";
const COLLECTION_NAME: &str = "decision_trees";

#[derive(Clone)]
pub struct DecisionTreeDao {
    collection: Collection<DecisionTree>
}

impl DecisionTreeDao {
    pub async fn new() -> DecisionTreeDao {
        match env::var(MONGODB_URI) {
            Ok(uri) => Self { collection: DecisionTreeDao::get_collection(&uri).await },
            Err(err) => panic!("{:?}", err)
        }
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<DecisionTree>, Error> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await
    }

    pub async fn save(&self, decision_tree: &DecisionTree) -> Result<UpdateResult, Error> {
        let filter = doc! { "_id": decision_tree._id };
        let update = doc! { "$set": decision_tree };
        let options = mongodb::options::UpdateOptions::builder().upsert(true).build();
        self.collection.update_one(filter, update, options).await
    }

    async fn get_collection(uri: &String) -> Collection<DecisionTree> {
        let client = Client::with_uri_str(uri).await.expect("MongoDB client creation failed");
        let db = client.database(DB_NAME);
        db.collection::<DecisionTree>(COLLECTION_NAME)
    }
}
