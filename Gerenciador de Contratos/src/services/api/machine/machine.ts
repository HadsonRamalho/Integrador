import { client } from "..";
import { Machine } from "@/interfaces/machine";

export async function listMachine(): Promise<Machine[]> {
  try {
    const response = await client.get<Machine[]>("/lista_todas_maquinas");

    if (response.status === 200) {
      const data = response.data;
      return data;
    }else {
      console.warn("Resposta inesperada:", response.status);
      throw new Error(`Erro ao listar as máquinas. Status code: ${response.status}`);
    }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao listar as máquinas: Código [${error.response?.status}]`
    );
  }
}

export const loadMachineImage = async (machineId: string): Promise<string> => {
  try {
    console.log("machineId: ", machineId);
    const response = await client.post<string>("/recupera_imagem_maquina", machineId, {
      headers: {
        'Content-Type': 'application/json'
      }
    });

    if (!response.data) {
      throw new Error("Erro ao carregar imagem da máquina");
    }

    const imagePath = response.data;

    const imageUrl = `https://g6v9psc0-3003.brs.devtunnels.ms${imagePath}`;
    return imageUrl;
  } catch (error) {
    console.error(error);
    throw error;
  }
};