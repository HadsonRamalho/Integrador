import { invoke } from "@tauri-apps/api";

export async function estruturaMaquina(nomemaquina, valoraluguel, numserie){
    try{      
      const maquina = await invoke("estrutura_maquina", {nomemaquina, valoraluguel, numserie});
      return maquina;
    }
    catch(error){
      console.log("[estruturaMaquina] : ", error);
    }
} 

export async function cadastraMaquina(nomemaquina, valoraluguel, numserie){
    try{
      const maquina = await estruturaMaquina(nomemaquina, valoraluguel, numserie);
      await invoke("cadastra_maquina", {maquina});
    } catch(error){
      console.log("[estruturaMaquina] : ", error);
    }
}