import { invoke } from "@tauri-apps/api";
import { error } from "pdf-lib";

export async function estruturaMaquina(nomemaquina, valoraluguel, numserie){
    try{      
      const maquina = await invoke("estrutura_maquina", {nomemaquina, valoraluguel, numserie});
      return maquina;
    }
    catch(error){
      console.log("[estruturaMaquina] : ", error);
      throw(error);
    }
} 

export async function cadastraMaquina(nomemaquina, valoraluguel, numserie){

  try{
    const response = await fetch('http://localhost:3000/estrutura_maquina', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({nomemaquina, valoraluguel, numserie})

    });

    if (!response.ok){
      const errorMessage = response.text();
      throw new Error(`Erro: ${errorMessage}`);
    }

    const maquina = await response.json();
    
    const responseCadastro = await fetch('http://localhost:3000/cadastra_maquina', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(maquina)

    });

    if (!responseCadastro.ok){
      const errorMessage = responseCadastro.text();
      throw new Error(`Erro: ${errorMessage}`);
    }

  } catch(error){
    console.log(error);
    throw(error);
  }
}

export const formataValor = (e) => {
  let valor = e.replace(/\D/g, "");
  valor = (Number(valor) / 100).toLocaleString("pt-BR", { style: "currency", currency: "BRL" });
  return valor;
};