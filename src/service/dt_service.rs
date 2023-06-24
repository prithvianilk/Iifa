use crate::domain::node::Node;
use crate::service::dt_dao::DtDao;

#[derive(Clone, Copy)]
pub struct DtService {
    pub dt_dao: DtDao
}

impl DtService {
    pub fn get_dt(&self) -> Node {
        let root = self.dt_dao.get_root();
        return root;
    }

    pub fn save_dt(&self, root: &Node) {
        self.dt_dao.save_dt(root);
    }
}
