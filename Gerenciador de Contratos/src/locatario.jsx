import { invoke } from "@tauri-apps/api/tauri";

export async function estruturaLocatario(idendereco, idsocio, cnpj, nomelocatario){
    try{      
      const locatario = await invoke("estrutura_locatario", {idendereco, idsocio, cnpj, nomelocatario});
      return locatario;
    }
    catch(error){
      console.log("[estruturaLocatario] : ", error);
    }
} 

export async function cadastraLocatario(idendereco, idsocio, cnpj, nomelocatario){
    try{
      const locatario = await estruturaLocatario(idendereco, idsocio, cnpj, nomelocatario);
      await invoke("cadastra_locatario", {locatario});
    } catch(error){
      console.log("[cadastraLocatario] : ", error);
    }
}