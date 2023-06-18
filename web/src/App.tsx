import React, { useCallback, useState } from 'react';
import logo from './logo.svg';
import './index.css';

import ReactFlow, { Controls, Background, Node, applyNodeChanges, applyEdgeChanges, NodeChange, EdgeChange, Edge } from 'reactflow';
import 'reactflow/dist/style.css';

function Flow() {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  useState(() => {
    const nodes: Node[] = [
      {
        id: '1',
        position: { x: 0, y: 0 },
        data: { label: 'Hello' },
      },
    ];
    fetch("http://localhost:8080/dt")
      .then(resp => resp.json())
      .then(resp => {
        console.log(resp);
      });
    setNodes(nodes);
  });

  const onNodesChange = useCallback((changes: NodeChange[]) => setNodes((nds) => applyNodeChanges(changes, nds)), []);
  const onEdgesChange = useCallback((changes: EdgeChange[]) => setEdges((eds) => applyEdgeChanges(changes, eds)), []);

  return (
    <div style={{ height: '100vh', width: "100%" }}>
      <ReactFlow
        nodes={nodes}
        onNodesChange={onNodesChange}
        edges={edges}
        onEdgesChange={onEdgesChange}
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
}


function App() {
  return (
    <Flow />
  );
}

export default App;
