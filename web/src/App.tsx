import React from "react";
import './index.css';
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import DecisionTreeTablePage from "./components/pages/DecisionTreeTablePage";
import DecisionTreeEditorPage from "./components/pages/DecisionTreeEditorPage";

const router = createBrowserRouter([
  {
    path: "/",
    element: <DecisionTreeTablePage />
  },
  {
    path: "/:decision_tree_id",
    element: <DecisionTreeEditorPage />
  },
]);

const App = () => {
  return <RouterProvider router={router} />
}

export default App;