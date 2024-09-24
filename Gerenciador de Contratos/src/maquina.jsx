import { invoke } from "@tauri-apps/api";

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
      const maquina = await estruturaMaquina(nomemaquina, valoraluguel, numserie);
      await invoke("cadastra_maquina", {maquina});
    } catch(error){
      console.log("[estruturaMaquina] : ", error);
      throw(error);
    }
}

export const formataValor = (e) => {
  let valor = e.replace(/\D/g, "");
  valor = (Number(valor) / 100).toLocaleString("pt-BR");
  return valor;
};