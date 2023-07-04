import React, { useEffect, useState } from "react";
import { DecisionTree } from "../../lib/DecisionTree";
import whompingWillowClient from "../../lib/whompingWillow.client";
import { useNavigate } from "react-router-dom";

const DecisionTreeTablePage = () => {
    const navigate = useNavigate();
    const [description, setDescription] = useState<string>("");
    const [decisionTrees, setDecisionTrees] = useState<DecisionTree[]>([]);

    const getAllDecisionTrees = async () => {
        const decisionTrees = await whompingWillowClient.getAllDecisionTrees();
        setDecisionTrees(decisionTrees);
    }

    const handleCreateNewDecsionTree = async () => {
        await whompingWillowClient.createNewDecisionTree(description);
        await getAllDecisionTrees();
    }

    useEffect(() => {
        getAllDecisionTrees();
    }, []);

    return (
        <div className="flex flex-col justify-center items-center h-screen overflow-x-auto">
            <table className="table w-1/2">
                <thead>
                    <tr>
                        <th>#</th>
                        <th>Description</th>
                    </tr>
                </thead>
                <tbody>
                    {decisionTrees.map(({ _id, description }, index) => (
                        <tr key={_id} className="hover" onClick={() => navigate(`/${_id}`)}>
                            <th>{index}</th>
                            <td>{description}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
            <input type="text" placeholder="Type new decision tree desc here" className="input w-1/3 my-7" value={description} onChange={e => setDescription(e.target.value)} />
            <button className="btn w-1/4" onClick={handleCreateNewDecsionTree}>Create new</button>
        </div>
    )
}

export default DecisionTreeTablePage;