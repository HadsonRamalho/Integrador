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
      throw(error);
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
      throw(error);
    }
}

export function widthstr(valor){
  let valorvh = valor + "vh";
  return valorvh
}

export function marginstr(valor){
  let valorstr = valor + "%";
  return valorstr
}

export const selecionaUf = (onUfChange, isReadOnly = false, widthvh = 65, marginleft = 9) => {
  return (
    <div className="input-box">
      <label htmlFor="estado"></label>
    <select
      name="estado"
      style={{width: widthstr(widthvh), marginLeft: marginstr(marginleft)}}
      onChange={(e) => onUfChange(e.target.value)} // Atualiza o valor selecionado
      disabled={isReadOnly} // Define se o campo é somente leitura
      required
      aria-label="Selecione o estado"
    >
        <option value="">Selecione o estado</option>
        <option value="AC">AC</option>
        <option value="AL">AL</option>
        <option value="AP">AP</option>
        <option value="AM">AM</option>
        <option value="BA">BA</option>
        <option value="CE">CE</option>
        <option value="ES">ES</option>
        <option value="GO">GO</option>
        <option value="MA">MA</option>
        <option value="MT">MT</option>
        <option value="MS">MS</option>
        <option value="MG">MG</option>
        <option value="PA">PA</option>
        <option value="PB">PB</option>
        <option value="PR">PR</option>
        <option value="PE">PE</option>
        <option value="PI">PI</option>
        <option value="RJ">RJ</option>
        <option value="RN">RN</option>
        <option value="RS">RS</option>
        <option value="RO">RO</option>
        <option value="RR">RR</option>
        <option value="SC">SC</option>
        <option value="SP">SP</option>
        <option value="SE">SE</option>
        <option value="TO">TO</option>
      </select>
    </div>
  );
};

export const selecionaUfDefinido = (onUfChange, selectedUf = "", isReadOnly = false) => {
  return (
    <div className="input-box">
      <label htmlFor="estadoDef"></label>
    <select
      id="estadoDef"
      name="estadoDef"
      value={selectedUf}
      onChange={(e) => onUfChange(e.target.value)} // Atualiza o valor selecionado
      disabled={isReadOnly} // Define se o campo é somente leitura
      required
      aria-label="Selecione o estado"
    >
        <option value="">Selecione o estado</option>
        <option value="AC">AC</option>
        <option value="AL">AL</option>
        <option value="AP">AP</option>
        <option value="AM">AM</option>
        <option value="BA">BA</option>
        <option value="CE">CE</option>
        <option value="ES">ES</option>
        <option value="GO">GO</option>
        <option value="MA">MA</option>
        <option value="MT">MT</option>
        <option value="MS">MS</option>
        <option value="MG">MG</option>
        <option value="PA">PA</option>
        <option value="PB">PB</option>
        <option value="PR">PR</option>
        <option value="PE">PE</option>
        <option value="PI">PI</option>
        <option value="RJ">RJ</option>
        <option value="RN">RN</option>
        <option value="RS">RS</option>
        <option value="RO">RO</option>
        <option value="RR">RR</option>
        <option value="SC">SC</option>
        <option value="SP">SP</option>
        <option value="SE">SE</option>
        <option value="TO">TO</option>
      </select>
    </div>
  );
};