import { Edge, Node } from 'reactflow';

export type GetDecisionTreeAsFlowResponse = {
    graph: Graph, 
    context: string
    description: string;
}

export type DecisionTree = {
    _id: string,
    description: string;
    graph: Graph
};

export type Graph = {
    nodes: Node[],
    edges: Edge[]
}