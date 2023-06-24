use crate::dt_dao::DtDao;
use crate::service::{react_flow_service::ReactFlowService, dt_service::DtService};

#[derive(Clone)]
pub struct AppData {
    pub react_flow_service: ReactFlowService,
    pub dt_service: DtService
}

pub fn get_app_data() -> AppData {
    let dt_dao: DtDao = DtDao{};
    let dt_service: DtService = DtService{dt_dao};
    let react_flow_service: ReactFlowService = ReactFlowService{dt_service};
    AppData{react_flow_service, dt_service: dt_service}
}