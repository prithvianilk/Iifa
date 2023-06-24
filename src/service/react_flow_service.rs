use crate::service::dt_service::DtService;
use crate::domain::node::Node;
use crate::dto::react_flow_dtos::*;

const STEP: f64 = 200.0;

#[derive(Clone)]
pub struct ReactFlowService {
    pub dt_service: DtService,
}

impl ReactFlowService {
    pub fn get_graph(&self) -> Graph {
        let root = self.dt_service.get_dt();
        let mut nodes: Vec<FlowNode> = Vec::new();
        let mut edges: Vec<FlowEdge>= Vec::new();
        self.populate_nodes_and_edges(&root, 1, &Position { x: 0.0, y: 0.0 }, &mut nodes, &mut edges);
        Graph{nodes, edges}
    }

    pub fn save_graph_as_dt(&self, graph: &Graph) {
        let root = self.construct_root(&graph);
        self.dt_service.save_dt(&root.unwrap())
    }

    fn populate_nodes_and_edges(&self, root: &Node, id: u32, position: &Position, nodes: &mut Vec<FlowNode>, edges: &mut Vec<FlowEdge>) {
        let is_terminal = root.value.is_some();
        if is_terminal {
            let data = Data { label: root.value.clone().unwrap(), description: root.description.clone(), value: root.value.clone(), predicate: root.predicate.clone() };
            nodes.push(FlowNode{id: id.to_string(), position: position.clone(), data});
            return
        }
        let data = Data{ label: root.description.clone(), description: root.description.clone(), value: root.value.clone(), predicate: root.predicate.clone() };
        nodes.push(FlowNode{id: id.to_string(), position: position.clone(), data });
        if let Some(left) = &root.left {
            let edge_id = format!("{}-{}", id, id * 2);
            edges.push(FlowEdge { id: edge_id, source: id.to_string(), target: (id * 2).to_string(), label: Direction::LEFT });
            self.populate_nodes_and_edges(left, id * 2, &Position { x: position.x - STEP, y: position.y + STEP }, nodes, edges)
        }
        if let Some(right) = &root.right {
            let edge_id = format!("{}-{}", id, id * 2 + 1);
            edges.push(FlowEdge { id: edge_id, source: id.to_string(), target: (id * 2 + 1).to_string(), label: Direction::RIGHT });
            self.populate_nodes_and_edges(right, id * 2 + 1, &Position { x: position.x + STEP, y: position.y + STEP }, nodes, edges)
        }
    }

    fn construct_root(&self, graph: &Graph) -> Option<Box<Node>> {
        let nodes = &graph.nodes;
        let edges = &graph.edges;

        let root_node = nodes.iter().find(|node| !self.has_incoming_edge(node, edges));
        if let Some(root_node) = root_node {
            Some(self.construct_subtree(root_node, nodes, edges))
        } else {
            None
        }
    }

    fn construct_subtree(&self, node: &FlowNode, nodes: &[FlowNode], edges: &[FlowEdge]) -> Box<Node> {
        let mut root = Node {
            description: node.data.description.clone(),
            predicate: node.data.predicate.clone(),
            value: node.data.value.clone(),
            left: None,
            right: None,
        };

        let outgoing_edges = edges.iter().filter(|edge| edge.source == node.id);
        for edge in outgoing_edges {
            if let Some(target_node) = nodes.iter().find(|node| node.id == edge.target) {
                let child_node = self.construct_subtree(target_node, nodes, edges);
                root.link(*child_node, edge.label == Direction::LEFT);
            }
        }

        Box::new(root)
    }

    fn has_incoming_edge(&self, node: &FlowNode, edges: &[FlowEdge]) -> bool {
        edges.iter().any(|edge| edge.target == node.id)
    }
}