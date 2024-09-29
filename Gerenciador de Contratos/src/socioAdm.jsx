import { invoke } from "@tauri-apps/api/tauri";
import {widthstr, marginstr} from "./endereco";

export async function estruturaSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade, idsocio = ""){
    try{
      if (idsocio == ""){
        const socio = await invoke("estrutura_socio_adm", {idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade});
        return socio;
      }
      const socio = await invoke("estrutura_primeiro_socio", {idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade, idsocio});
      return socio;
    } catch(error) {
      console.log("[estruturaSocioAdm] : ", error);
      throw(error);
    }
}

export async function cadastraSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade, idsocio = ""){
    try{
      const socioadm = await estruturaSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade, idsocio);
      const idsocio_ = await invoke("cadastra_socio_adm", {socioadm});
      return idsocio_;
    } catch(error){
      console.log("[cadastraSocioAdm] : ", error);
      throw(error);
    }
}

export async function cadastraPrimeiroSocioLocadora(idsocio, idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade){
  try{
    const socioadm = await invoke("");
  } catch(error){
    console.log(error);
  }
}

export const selecionaNacionalidade = (setNacionalidade, widthvh = 65, marginleft = 9) => {
  return (
    <div className= "input-box">
            <select id= "nacionalidade" name= "nacionalidade" required aria-label= "Nacionalidade do sócio Administrativo"
              style={{width: widthstr(widthvh), marginLeft: marginstr(marginleft)}}
              onChange={(e) => setNacionalidade(e.currentTarget.value)}>
                <option value="" disabled selected> Selecione a nacionalidade </option>
                <option value = "Brasil"> Brasileiro </option>
                <option value = "EUA"> Americano </option>
                <option value = "Argentina"> Argentino </option>
                <option value = "Chile"> Chileno </option>
                <option value = "China"> Chinês </option>
                <option value = "Coreia"> Coreano </option>
            </select>
        </div>
  );
};

export const selecionaEstadoCivil = (setEstadoCivil, widthvh = 65, marginleft = 9) => {
  return (
    <div class= "input-box">
            <select id= "estadoCivil" name= "estadoCivil" required aria-label= "Estado civil do socio Administrativo"
              style={{width: widthstr(widthvh), marginLeft: marginstr(marginleft)}}
              onChange={(e) => setEstadoCivil(e.currentTarget.value)}>
                <option value="" disabled selected> Selecione seu estado civil </option>
                <option value = "Solteiro"> Solteiro(a) </option>
                <option value = "Casado"> Casado(a) </option>
                <option value = "Viuvo"> Viuvo(a) </option>
                <option value = "Divorciado"> Divorciado(a) </option>
            </select>
        </div>
  );
};