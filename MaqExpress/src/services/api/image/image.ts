import { ImgOutput, MachineImage } from "@/interfaces/image";
import { client } from "..";

export async function createImage(info: FormData): Promise<ImgOutput>{
  try {
    const response = await client.post<ImgOutput>(`/cadastra_imagem`, info);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao cadastrar a imagem. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao cadastrar informações da imagem: Código [${error.response?.status}]`
    );
  }
}

export async function createMachineImage(info: MachineImage): Promise<ImgOutput>{
  try {
    const response = await client.post<ImgOutput>(`/cadastra_imagem_maquina`, info);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao cadastrar a imagem da máquina. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao cadastrar informações da imagem da máquina: Código [${error.response?.status}]`
    );
  }
}

