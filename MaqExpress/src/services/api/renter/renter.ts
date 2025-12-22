import { Renter } from "@/interfaces/renter";
import { client } from "..";

export async function loadRenterByUserId(
  id: string
):Promise<Renter>{
  try {
    const url = `/busca_locatario_idusuario/?id=${encodeURIComponent(id)}`;
    const response = await client.get<Renter>(url);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar o locatário. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar o locatário:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar o locatário: ${error.response?.status || "sem status"}`
    );
  }  
}

export async function createRenter(idusuario:string, idendereco: string): Promise<string> {
  try {
    const response = await client.post<string>("/cadastra_locatario", {
      idusuario: idusuario,
      idendereco: idendereco,
    });

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao cadastrar o locatário. Status code: ${response.status}`);    
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao cadastrar o locatário: Código [${error.response?.status}]`
    );
  }
}