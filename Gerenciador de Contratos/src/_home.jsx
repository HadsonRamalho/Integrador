import React from "react";
import ReactDOM from "react-dom/client";
import Home from "./home";
import "./styles.css";

localStorage.removeItem('token');


ReactDOM.createRoot(document.getElementById("rootHome")).render(
    <Home />
);