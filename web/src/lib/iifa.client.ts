import { Node, Edge } from "reactflow";

async function evaluate(customerParams: string) {
    await fetch('http://localhost:8080/dt/evaluate', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: customerParams,
    })
        .then(response => response.json())
        .then(data => alert(data.result))
        .catch(alert);
}

async function saveDecisionTree(nodes: Node[], edges: Edge[]) {
    await fetch('http://localhost:8080/react-flow/dt', {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            nodes,
            edges
        }),
    })
        .then(response => response.json())
        .then(data => console.log(data))
        .catch(console.log);
}


async function getDecisionTree() {
    return await fetch("http://localhost:8080/react-flow/dt").
        then(resp => resp.json());
}

export default { evaluate, saveDecisionTree, getDecisionTree };