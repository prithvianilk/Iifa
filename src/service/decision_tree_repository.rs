use std::env;

use mongodb::bson::{doc, Uuid};
use mongodb::{Client, Collection};
use mongodb::error::Error;

use crate::domain::decision_tree::DecisionTree;
use futures::stream::TryStreamExt;

const MONGODB_URI: &str = "MONGODB_URI";
const DB_NAME: &str = "whomping_willow";
const COLLECTION_NAME: &str = "decision_trees";

#[derive(Clone)]
pub struct DecisionTreeRepository {
    collection: Collection<DecisionTree>
}

impl DecisionTreeRepository {
    pub async fn new() -> DecisionTreeRepository {
        match env::var(MONGODB_URI) {
            Ok(uri) => Self { collection: DecisionTreeRepository::get_collection(&uri).await },
            Err(err) => panic!("{:?}", err)
        }
    }

    pub async fn get_all(&self) -> Result<Vec<DecisionTree>, Error> {
        let mut decision_trees: Vec<DecisionTree> = Vec::new();
        let mut cursor = self.collection.find(None, None).await?; 
        while let Some(decision_tree) = cursor.try_next().await? {
            decision_trees.push(decision_tree);
        }
        Ok(decision_trees)
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<DecisionTree>, Error> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await
    }

    pub async fn save(&self, decision_tree: &DecisionTree) -> Result<(), Error> {
        let filter = doc! { "_id": decision_tree._id };
        let update = doc! { "$set": decision_tree };
        let options = mongodb::options::FindOneAndUpdateOptions::builder().upsert(true).build();
        self.collection.find_one_and_update(filter, update, options).await.map(|_| ())
    }

    async fn get_collection(uri: &String) -> Collection<DecisionTree> {
        let client = Client::with_uri_str(uri).await.expect("MongoDB client creation failed");
        let db = client.database(DB_NAME);
        db.collection::<DecisionTree>(COLLECTION_NAME)
    }
}
