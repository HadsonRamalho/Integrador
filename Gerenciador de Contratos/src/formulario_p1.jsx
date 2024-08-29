import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";


function FormularioP1(){
  const [nomeLocadora, setNomeLocadora] = useState("");
  const [cnpj, setCnpj] = useState("");
  const [numero, setNumero] = useState("");
  const [endereco, setEndereco] = useState("");
  const [cidade, setCidade] = useState("");
    return (
        <div class= "conteudo">
        <h1>Locadora</h1>
        <form class="conteudo">
        <div class="input-group">
            <div class= "input-box">
                <label for= "nomeLocadora"> Nome da Locadora: </label>
                <input required
                onChange={(e) => setNomeLocadora(e.currentTarget.value)}
                placeholder="Nome da Locadora " 
                />
            </div>
            
            <div class= "input-box">
                <label for= "cnpj"> CNPJ: </label>
                <input required
                onChange={(e) => setCnpj(e.currentTarget.value)}
                placeholder="CNPJ da Locadora" 
            />
            </div>

            <div class= "input-box">
            <label for= "logradouro"> Logradouro: </label>
            <input id= "logradouro" type= "text" name= "logradouro" required aria-label= "Logradouro da locadora">
            </input></div>

            <div class= "input-box">
            <label for= "numero"> Numero: </label>
            <input id= "numero" type= "text" name= "numero" required aria-label= "Numero da locadora">
            </input></div>

            <div class= "input-box">
            <label for= "bairro"> Bairro: </label>
            <input id= "bairro" type= "text" name= "bairro" required aria-label= "Bairro da locadora">
            </input></div>

            <div class= "input-box">
                <label for= "cep"> CEP: </label>
                <input id= "cep" type= "text" name= "cep" required aria-label= "CEP da locadora ">
                </input></div>

            <div class= "input-box">
                <label for= "cidade"> Cidade: </label>
                <input id= "cidade" type= "text" name= "cidade" required aria-label= "Cidade da locadora ">
                </input></div>

                <div class="input-box">
                    <label for="estadoLocadora">Estado (UF)</label>
                    <select id="estadoLocadora" name="estadoLocadora" required aria-label="Selecione o estado da Locadora">
                        <option value="" disabled selected>Selecione o estado da Locadora</option>
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

        <p> Sócio Administrador da Locadora: </p>



        <div class= "input-box">
            <label for= "name"> Nome: </label>
            <input id= "name" type= "text" name= "name" required aria-label= "Nome do sócio Administrativo">
            </input></div>


        <div class= "input-box">
            <label for= "nacionalidade"> Nacionalidade </label>
            <select id= "nacionalidade" name= "nacionalidade" required aria-label= "Nacionalidade do sócio Administrativo">
                <option value="" disabled selected> Selecione sua nacionalidade </option>
                <option value = "Brasil"> Brasi </option>
                <option value = "EUA"> Estados Unidos </option>
                <option value = "Argentina"> Argentina </option>
                <option value = "Chile"> Chile </option>
            </select>
        </div>

        <div class= "input-box">
            <label for= "estadoCivil"> Estado Civil </label>
            <select id= "estadoCivil" name= "estadoCivil" required aria-label= "Estado civil do socio Administrativo">
                <option value="" disabled selected> Selecione seu estado civil </option>
                <option value = "Solteiro"> Solteiro(a) </option>
                <option value = "Casado"> Casado(a) </option>
                <option value = "Viuvo"> Viuvo(a) </option>
                <option value = "Divorciado"> Divorciado(a) </option>
            </select>
        </div>

        <div class= "input-box">
            <label for= "cpf"> CPF: </label>
            <input id= "cpf" type= "text" name= "cpf" required aria-label= "CPF do socio Administrativo">
            </input></div>

        <div class= "input-box">
            <label for= "orgaoEmissor"> Órgão Emissor do CPF</label>
            <input id= "orgaoEmissor" type= "text" name= "orgaoEmissor" required aria-label= "Orgão emissor do CPF do socio Administrativo">
            </input></div>

            <div class= "input-box">
            <label for= "logradouro"> Logradouro: </label>
            <input id= "logradouro" type= "text" name= "logradouro" required aria-label= "Logradouro">
            </input></div>

            <div class= "input-box">
            <label for= "numero"> Numero: </label>
            <input id= "numero" type= "text" name= "numero" required aria-label= "Numero">
            </input></div>

            <div class= "input-box">
            <label for= "bairro"> Bairro: </label>
            <input id= "bairro" type= "text" name= "bairro" required aria-label= "Bairro">
            </input></div>

            <div class= "input-box">
                <label for= "cep"> CEP: </label>
                <input id= "cep" type= "text" name= "cep" required aria-label= "CEP ">
                </input></div>

            <div class= "input-box">
                <label for= "cidade"> Cidade: </label>
                <input id= "cidade" type= "text" name= "cidade" required aria-label= "Cidade ">
                </input></div>

                <div class="input-box">
                    <label for="estado">Estado (UF)</label>
                    <select id="estado" name="estado" required aria-label="Selecione seu estado">
                        <option value="" disabled selected>Selecione seu estado</option>
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
    </div>

    <button type="submit" class="button">Continuar</button>
    </form>
    </div> 
    );
  }

  export default FormularioP1;