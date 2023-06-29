use mongodb::bson::Uuid;

use crate::domain::decision_tree::DecisionTree;
use crate::error::AppError;
use crate::service::decision_tree_dao::DecisionTreeDao;

#[derive(Clone)]
pub struct DecisionTreeService {
    pub dt_dao: DecisionTreeDao
}

impl DecisionTreeService {
    pub async fn get_decision_tree_by_id(&self, id: &Uuid) -> Result<DecisionTree, AppError> {
        match self.dt_dao.get_by_id(id).await {
            Ok(value) => self.safely_get_decision_tree(value),
            Err(err) => Err(AppError::GetDecisonTreeFailed { message: err.to_string() })
        }
    }

    pub async fn upsert_decision_tree(&self, decision_tree: &DecisionTree) -> Result<(), AppError>   {
        match self.dt_dao.save(decision_tree).await {
            Ok(_)=> Ok(()),
            Err(err) => Err(AppError::SaveDecisionTreeFailed { message: err.to_string() })
        }
    }

    fn safely_get_decision_tree(&self, value: Option<DecisionTree>) -> Result<DecisionTree, AppError> {
        match value {
            Some(value) => Ok(value),
            None => Err(AppError::GetDecisonTreeFailed { message: "No decision tree with given _id".to_string() }),
        }
    }
}
