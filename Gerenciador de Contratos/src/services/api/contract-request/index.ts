import { ContractRequest } from "@/interfaces/contract-request";
import { client } from "..";

export async function createContractRequest(
  idlocador: string,
  idlocatario: string,
  idmaquina: string,
  prazolocacao: number,
  medidatempolocacao: string,
  origemsolicitacao: string,
  valorsolicitacao: number,
): Promise<string> {
try {
  const response = await client.post<string>("/cadastra_solicitacao_contrato", {
    idlocador: idlocador,
    idlocatario: idlocatario,
    prazolocacao: prazolocacao,
    idmaquina: idmaquina,
    medidatempolocacao: medidatempolocacao,
    origemsolicitacao: origemsolicitacao,
    valorsolicitacao: valorsolicitacao
  });

  if (response.status === 200) {
    const data = response.data;
    return data;
  }
  console.warn("Resposta inesperada:", response.status);
  throw new Error(`Erro ao cadastrar a solicitação de contrato. Status code: ${response.status}`);    
// eslint-disable-next-line @typescript-eslint/no-explicit-any
} catch (error: any) {
  console.error(error.response?.status, error.message);
  throw new Error(
    `Falha ao cadastrar a solicitação de contrato: Código [${error.response?.status}]`
  );
}
}

export async function loadContractRequestById(
  id: string
):Promise<ContractRequest>{
  try {
    const url = `/busca_solicitacao_idsolicitacao/?id=${encodeURIComponent(id)}`;
    const response = await client.get<ContractRequest>(url);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar a solicitação. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar a solicitação:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar a solicitação: ${error.response?.status || "sem status"}`
    );
  }  
}

export async function loadContractRequestsByOwnerId(
  id: string
):Promise<ContractRequest[]>{
  try {
    const url = `/busca_solicitacoes_idlocador/?id=${encodeURIComponent(id)}`;
    const response = await client.get<ContractRequest[]>(url);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar as solicitações. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar as solicitações:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar as solicitações: ${error.response?.status || "sem status"}`
    );
  }  
}

export async function updateContractRequestStatus(
  id: string,
  status: string
):Promise<ContractRequest>{
  try {
    const url = `/atualiza_status_solicitacao`;
    const response = await client.patch<ContractRequest>(url, {
      id: id,
      status: status
    });

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao atualizar a solicitação. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao atualizar a solicitação:", error.response?.status, error.message);
    throw new Error(
      `Falha ao atualizar a solicitação: ${error.response?.status || "sem status"}`
    );
  }  
}