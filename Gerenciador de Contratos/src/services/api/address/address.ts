import { Address, CreateUserAddress } from "@/interfaces/address";
import { client } from "..";

export async function createUserAddress(info: CreateUserAddress){
  try {
    const response = await client.post<Address>(`/cadastra_endereco_usuario`, info);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao cadastrar o endereço. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao cadastrar informações do endereço: Código [${error.response?.status}]`
    );
  }
}

export async function loadAddressUserId(id: string): Promise<Address> {
  try {
    const response = await client.get<Address>(`/busca_endereco_idusuario/?idusuario=${encodeURIComponent(id)}`);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao carregar o endereço. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar informações do endereço: Código [${error.response?.status}]`
    );
  }
}

export async function loadAddressByCep(cep: string): Promise<unknown> {
  try {
    const res = await fetch(`https://viacep.com.br/ws/${cep}/json/`,
   {method:'GET'} );
   if(!res.ok){
    throw new Error( await res.text());
   }
   const endereco = await res.json();
   return endereco;
  } catch (error) {
    console.error(error);
  }
}

export async function updateAddress(address: Address): Promise<Address> {
  try {
    const response = await client.patch<Address>(`/atualiza_endereco`, address);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao atualizar o endereço. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao atualizar informações do endereço: Código [${error.response?.status}]`
    );
  }
}