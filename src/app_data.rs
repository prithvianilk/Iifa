use crate::service::{react_flow_service::ReactFlowService, decison_tree_service::DecisionTreeService, decision_tree_repository::DecisionTreeRepository};

#[derive(Clone)]
pub struct AppData {
    pub react_flow_service: ReactFlowService,
    pub decision_tree_service: DecisionTreeService
}

pub async fn get_app_data() -> AppData {
    let decision_tree_repository: DecisionTreeRepository = DecisionTreeRepository::new().await;
    let decision_tree_service: DecisionTreeService = DecisionTreeService{decision_tree_repository};
    let react_flow_service: ReactFlowService = ReactFlowService{};
    AppData{react_flow_service, decision_tree_service}
}