import { invoke } from "@tauri-apps/api/tauri";

export async function estruturaSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade){
    try{
      const socio = await invoke("estrutura_socio_adm", {idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade});
      return socio;
    } catch(error) {
      console.log("[estruturaSocioAdm] : ", error);
      throw(error);
    }
}

export async function cadastraSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade){
    try{
      const socioadm = await estruturaSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade);
      const idsocio = await invoke("cadastra_socio_adm", {socioadm});
      return idsocio;
    } catch(error){
      console.log("[cadastraSocioAdm] : ", error);
      throw(error);
    }
}