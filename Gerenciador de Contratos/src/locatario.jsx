import { invoke } from "@tauri-apps/api/tauri";

export async function estruturaLocatario(idendereco, idsocio, cnpj, nomelocatario){
    try{      
      const locatario = await invoke("estrutura_locatario", {idendereco, idsocio, cnpj, nomelocatario});
      return locatario;
    }
    catch(error){
      console.log("[estruturaLocatario] : ", error);
      throw(error);
    }
} 

export async function cadastraLocatario(idendereco, idsocio, cnpj, nomelocatario){
    try{
      const locatario = await estruturaLocatario(idendereco, idsocio, cnpj, nomelocatario);
      await invoke("cadastra_locatario", {locatario});
    } catch(error){
      console.log("[cadastraLocatario] : ", error);
      throw(error);
    }
}

export async function buscaLocatarioNome(nomelocatario){
    try{
        const locatario = await invoke("busca_locatario_nome", {nomelocatario});
        return locatario
    } catch(error){
        console.log(error);
        throw(error);
    }
  }

export async function buscaLocatarioCnpj(cnpj){
    try{
        const locatario = await invoke("busca_locatario_cnpj", {cnpj});
        return locatario;
    } catch(error){
        console.log(error);
        throw(error);
    }
  }