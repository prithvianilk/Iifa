import DecisionTree, { GetDecisionTreeAsFlowResponse, Graph } from "./DecisionTree";

const whompingWillowServiceBaseUrl = "http://localhost:8080";

async function evaluate(_id: string, inputContext: string) {
    const url = `${whompingWillowServiceBaseUrl}/decision_trees/${_id}/evaluate`;
    await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: inputContext,
    })
        .then(response => response.json())
        .then(data => alert(data.result))
        .catch(alert);
}

async function saveDecisionTreeFromFlow(decisionTree: DecisionTree) {
    const url = `${whompingWillowServiceBaseUrl}/react-flow/decision_trees`;
    await fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(decisionTree),
    })
        .then(response => response.json())
        .catch(console.log);
}

async function getDecisionTreeAsFlow(_id: String): Promise<GetDecisionTreeAsFlowResponse> {
    const url = `${whompingWillowServiceBaseUrl}/react-flow/decision_trees/${_id}`;
    return await fetch(url).then(resp => resp.json());
}

export default { evaluate, saveDecisionTree: saveDecisionTreeFromFlow, getDecisionTree: getDecisionTreeAsFlow };
