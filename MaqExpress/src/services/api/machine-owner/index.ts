import { User } from "@/interfaces/user";
import { client } from "..";

export async function loadMachineOwnerByMachineId(
  id: string
):Promise<User>{
  try {
    const url = `/busca_usuario_idmaquina/?id=${encodeURIComponent(id)}`;
    const response = await client.get<User>(url);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar o dono da máquina. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar o dono da máquina:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar o dono da máquina: ${error.response?.status || "sem status"}`
    );
  }  
}