import { Edge, Node } from 'reactflow';

type DecisionTree = {
    _id: String,
    graph: Graph
};

export type Graph = {
    nodes: Node[],
    edges: Edge[]
}

export default DecisionTree;