use mongodb::bson::Uuid;

use crate::domain::decision_tree::DecisionTree;
use crate::error::AppError;
use crate::service::decision_tree_repository::DecisionTreeRepository;

#[derive(Clone)]
pub struct DecisionTreeService {
    pub decision_tree_repository: DecisionTreeRepository
}

impl DecisionTreeService {
    pub async fn get_all_decision_trees(&self) -> Result<Vec<DecisionTree>, AppError> {
        self.decision_tree_repository.get_all().await
                                     .map_err(|err| AppError::GetDecisonTreeFailed { message: err.to_string() })
    }

    pub async fn get_decision_tree_by_id(&self, id: &Uuid) -> Result<DecisionTree, AppError> {
        match self.decision_tree_repository.get_by_id(id).await {
            Ok(value) => self.safely_get_decision_tree(value),
            Err(err) => Err(AppError::GetDecisonTreeFailed { message: err.to_string() })
        }
    }

    pub async fn upsert_decision_tree(&self, decision_tree: &DecisionTree) -> Result<(), AppError>   {
        self.decision_tree_repository.save(decision_tree).await
            .map(|_| ())
            .map_err(|err| AppError::SaveDecisionTreeFailed { message: err.to_string() })
    }

    fn safely_get_decision_tree(&self, value: Option<DecisionTree>) -> Result<DecisionTree, AppError> {
        value.map(|value| Ok(value))
             .unwrap_or(Err(AppError::GetDecisonTreeFailed { message: "No decision tree with given _id".to_string() }))
    }
}
