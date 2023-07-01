import { Edge, Node } from 'reactflow';

export type GetDecisionTreeAsFlowResponse = {
    graph: Graph, 
    context: string
}

type DecisionTree = {
    _id: String,
    graph: Graph
};

export type Graph = {
    nodes: Node[],
    edges: Edge[]
}

export default DecisionTree;