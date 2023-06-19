import React, { useCallback, useState } from 'react';
import './index.css';
import ReactFlow, { Controls, Background, Node, applyNodeChanges, applyEdgeChanges, NodeChange, EdgeChange, Edge, addEdge } from 'reactflow';
import 'reactflow/dist/style.css';

declare global {
  interface Window {
    edgeModal: HTMLDialogElement;
  }
}

type Predicate = "AppliedBeforeDeadline" | "SalaryAbove" | "AgeAbove";
const predicates: Predicate[] = ["AppliedBeforeDeadline", "SalaryAbove", "AgeAbove"];

function Flow() {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const onConnect = useCallback(
    (connection: any) => setEdges((eds) => addEdge(connection, eds)),
    [setEdges]
  );
  const [selectedEdge, setSelectedEdge] = useState<Edge | null>(null);

  const [description, setDescription] = useState<string>("");
  const [selectedEdgeDirection, setSelectedEdgeDirection] = useState<string>("");
  const [isPredicate, setIsPredicate] = useState<boolean>(false);
  const [predicateType, setPredicateType] = useState<Predicate>('AgeAbove');
  const [predicateData, setPredicateData] = useState<{ [x: string]: any }>({});
  const [value, setValue] = useState<string | null>(null);
  const [customerParams, setCustomerParams] = useState<string>("");

  const saveDt = () => {
    fetch('http://localhost:8080/react-flow/dt', {
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

  const evaluate = () => {
    fetch('http://localhost:8080/dt/evaluate', {
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


  useState(() => {
    fetch("http://localhost:8080/react-flow/dt")
      .then(resp => resp.json())
      .then(({ nodes, edges }) => {
        console.log(nodes, edges);
        setNodes(nodes);
        setEdges(edges);
      });
    setNodes(nodes);
  });

  const onNodesChange = useCallback((changes: NodeChange[]) => setNodes((nds) => applyNodeChanges(changes, nds)), []);
  const onEdgesChange = useCallback((changes: EdgeChange[]) => setEdges((eds) => applyEdgeChanges(changes, eds)), []);

  const handleEdgeClick = (_: any, edge: Edge) => {
    setSelectedEdgeDirection(edge.label as string);
    setSelectedEdge(edge);
    window.edgeModal.showModal();
  };

  return (
    <div className='p-5' style={{ display: 'flex', height: '100vh' }}>
      <div className='p-2 w-4/5'>
        <ReactFlow
          nodes={nodes}
          onNodesChange={onNodesChange}
          edges={edges}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          onEdgeClick={handleEdgeClick}
        >
          <Background />
          <Controls />
        </ReactFlow>
      </div>
      <div className='p-2 h-screen w-1/5'>
        <div>
          <label>Description</label>
          <input type="text" placeholder="Type here" className="input w-full max-w-xs" value={description} onChange={(e) => setDescription(e.target.value)} />
        </div>
        <div>
          <label>Predicate?</label>
          <br />
          <input type="checkbox" className="toggle" checked={isPredicate} onClick={() => setIsPredicate(!isPredicate)} />
        </div>
        {isPredicate ? (
          <div>
            <select className="select w-full max-w-xs"
              onChange={(e) => {
                const selectedValue = e.currentTarget.value;
                setPredicateType(selectedValue as Predicate);
              }} value={predicateType}>
              <option disabled selected >Predicate Type</option>
              {predicates.map(value => (<option>{value}</option>))}
            </select>
            <div>
              {predicateType === 'AgeAbove' && (
                <div>
                  <label>Min Age</label>
                  <input type="text" placeholder="Type here" className="input w-full max-w-xs" value={predicateData.min_age} onChange={(e) => setPredicateData({ "min_age": Number.parseInt(e.target.value) })} />
                </div>
              )}
              {predicateType === 'SalaryAbove' && (
                <div>
                  <label>Min Salary</label>
                  <input type="text" placeholder="Type here" className="input w-full max-w-xs" value={predicateData.min_salary} onChange={(e) => setPredicateData({ "min_salary": Number.parseInt(e.target.value) })} />
                </div>
              )}
              {predicateType === 'AppliedBeforeDeadline' && (
                <div>
                  <label>Deadline</label>
                  <input type="text" placeholder="Type here" className="input w-full max-w-xs" value={predicateData.application_deadline} onChange={(e) => setPredicateData({ "application_deadline": e.target.value })} />
                </div>
              )}
            </div>
          </div>
        ) : (
          <div>
            <label>Value</label>
            <input type="text" placeholder="Type here" className="input w-full max-w-xs" value={value || ""} onChange={(e) => setValue(e.target.value)} />
          </div>
        )}

        <button className="btn w-full" onClick={() => {
          let defaultNode = {};
          if (isPredicate) {
            defaultNode = { predicate: { [predicateType]: predicateData } }
          } else {
            defaultNode = { predicate: "DEFAULT" }
          }
          setNodes([...nodes, {
            id: (nodes.length * 100).toString(),
            position: { x: 0, y: -100 },
            data: { ...defaultNode, label: description, value, description }
          }]);
          setDescription("");
          setValue("");
          setIsPredicate(false);
          setPredicateData({});
        }}>
          Create Node
        </button>
        <button className="mt-10 btn w-full" onClick={saveDt}>
          Save
        </button>
        <textarea className="textarea mt-10 w-full" placeholder="" value={customerParams} onChange={e => setCustomerParams(e.target.value)} />
        <button className="mt-10 btn w-full" onClick={evaluate}>
          Evaluate
        </button>
      </div>
      <dialog id="edgeModal" className="modal">
        <form method="dialog" className="modal-box">
          <button className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
          <h3 className="font-bold text-lg">Edge editor</h3>
          <p className="py-4">Direction: {selectedEdge?.label}</p>
          <input type="text" placeholder="Type here" className="input w-full max-w-xs" value={selectedEdgeDirection} onChange={(e) => setSelectedEdgeDirection(e.target.value)} />
          <button className="btn w-full" onClick={() => {
            setEdges([...edges.map((edge) => edge.id === selectedEdge?.id ? { ...edge, label: selectedEdgeDirection } : edge)])
          }}>
            Save
          </button>
        </form>
      </dialog>
    </div >
  );
}


function App() {
  return <Flow />;
}

export default App;
