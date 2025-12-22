import { Contract, ContractPDF } from "@/interfaces/contract";
import { client } from "..";

export async function loadPdfByRequestId(
  id: string
):Promise<ContractPDF>{
  try {
    const url = `/gera_contrato_idsolicitacao/?id=${encodeURIComponent(id)}`;
    const response = await client.get<ContractPDF>(url);

    if (response.status === 200) {
      console.log('contrato: ', response.data);
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar o contrato. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar o contrato:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar o contrato: ${error.response?.status || "sem status"}`
    );
  }  
}

export async function loadContractByRequestId(
  id: string
):Promise<Contract>{
  try {
    const url = `/busca_contrato_idsolicitacao/?id=${encodeURIComponent(id)}`;
    const response = await client.get<Contract>(url);

    if (response.status === 200) {
      console.log('contrato: ', response.data);
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar o contrato. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar o contrato:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar o contrato: ${error.response?.status || "sem status"}`
    );
  }  
}