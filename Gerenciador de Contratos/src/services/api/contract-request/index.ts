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