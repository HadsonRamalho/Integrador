import { invoke } from "@tauri-apps/api/tauri";

export async function estruturaEndereco(cep, logradouro, numeroendereco, complemento, cidade, uf){
    try{
      const endereco = await invoke("estrutura_endereco", {
        logradouro, 
        cep, 
        complemento, 
        numeroendereco, 
        cidade, 
        uf
      })
      return endereco;
    }
    catch(error){
      console.log("[estruturaEndereco] : ", error);
    }
}

export async function cadastraEndereco(cep, logradouro, numeroendereco, complemento, cidade, uf){
    const endereco = await estruturaEndereco(cep, logradouro, numeroendereco, complemento, cidade, uf);
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      console.log("Endereço foi cadastrado");
      return idendereco;
    } catch(error){
      console.log("[cadastraEndereco] : ", error);
    }
}