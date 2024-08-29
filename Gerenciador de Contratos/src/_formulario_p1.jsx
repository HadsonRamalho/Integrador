import React from "react";
import ReactDOM from "react-dom/client";
import FormularioP1 from "./formulario_p1";
import "./formulario.css";

ReactDOM.createRoot(document.getElementById("root")).render(
    <FormularioP1 />
);

document.getElementById('estadoForm').addEventListener('submit', function(event) {
    // Captura os valores dos dois selects
    const estadoSocioLocadora = document.getElementById('estadoSocioLocadora').value;

    // Chama a função com os dois estados selecionados
 
    console.log(estadoLocadora);

});