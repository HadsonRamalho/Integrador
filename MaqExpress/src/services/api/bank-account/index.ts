import { BankAccount } from "@/interfaces/bank-account";
import { client } from "..";

export async function loadBankAccountByUserId(
  id: string
):Promise<BankAccount>{
  try {
    const url = `/busca_conta_bancaria_idusuario/?id=${encodeURIComponent(id)}`;
    const response = await client.get<BankAccount>(url);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar a conta bancária. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar a conta bancária:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar a conta bancária: ${error.response?.status || "sem status"}`
    );
  }  
}

export async function createBankAccount(idusuario:string, numeroconta: string,
    numeroagencia: string, nomebanco: string
): Promise<string> {
  try {
    const response = await client.post<string>("/cadastra_conta_bancaria", {
      idusuario: idusuario,
      numeroconta: numeroconta,
      numeroagencia: numeroagencia,
      nomebanco: nomebanco
    });

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao cadastrar a conta bancária. Status code: ${response.status}`);    
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao cadastrar a conta bancária: Código [${error.response?.status}]`
    );
  }
}

export async function updateBankAccount(
  account: BankAccount
):Promise<BankAccount>{
  try {
    const url = `/atualiza_conta_bancaria`;
    const response = await client.put<BankAccount>(url, account);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao atualizar a conta bancária. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao atualizar a conta bancária:", error.response?.status, error.message);
    throw new Error(
      `Falha ao atualizar a conta bancária: ${error.response?.status || "sem status"}`
    );
  }  
}