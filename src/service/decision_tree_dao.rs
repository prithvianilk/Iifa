use mongodb::bson::{doc, Uuid};
use mongodb::results::UpdateResult;
use mongodb::{Client, Collection};
use mongodb::error::Error;

use crate::domain::decision_tree::DecisionTree;

#[derive(Clone)]
pub struct DecisionTreeDao {
    collection: Collection<DecisionTree>
}

impl DecisionTreeDao {
    pub async fn new() -> DecisionTreeDao {
        Self { collection: DecisionTreeDao::get_collection().await }
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

    async fn get_collection() -> Collection<DecisionTree> {
        let client = Client::with_uri_str("mongodb+srv://thepirateking3d2y:hwuqjioLdvEWmZ2D@clusta-0.bn4ul2j.mongodb.net/?retryWrites=true&w=majority").await.expect("msg");
        let db = client.database("whomping_willow");
        db.collection::<DecisionTree>("decision_trees")
    }
}