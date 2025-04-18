import { client } from "..";
import {
  Machine,
  MachineIds,
  MachineInput,
  MachineRentValue,
} from "@/interfaces/machine";

export async function listMachine(): Promise<Machine[]> {
  try {
    const response = await client.get<Machine[]>("/lista_todas_maquinas");

    if (response.status === 200) {
      const data = response.data;
      return data;
    } else {
      console.warn("Resposta inesperada:", response.status);
      throw new Error(
        `Erro ao listar as máquinas. Status code: ${response.status}`,
      );
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao listar as máquinas: Código [${error.response?.status}]`,
    );
  }
}

export const loadMachineImage = async (machineId: string): Promise<string> => {
  try {
    console.log("machineId: ", machineId);
    const response = await client.post<string>(
      "/recupera_imagem_maquina",
      machineId,
      {
        headers: {
          "Content-Type": "application/json",
        },
      },
    );

    if (!response.data) {
      throw new Error("Erro ao carregar imagem da máquina");
    }

    const imagePath = response.data;

    const imageUrl = `${import.meta.env.VITE_URL_BASE}${imagePath}`;
    console.log(imageUrl);
    return imageUrl;
  } catch (error) {
    console.error(error);
    throw error;
  }
};

export async function loadMachinePublicId(id: string): Promise<Machine> {
  try {
    const response = await client.get<Machine>(
      `/busca_maquina_idpublico/?id=${encodeURIComponent(id)}`,
    );

    if (response.status === 200) {
      const data = response.data;
      return data;
    } else {
      console.warn("Resposta inesperada:", response.status);
      throw new Error(
        `Erro ao carregar a máquina. Status code: ${response.status}`,
      );
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar informações da máquina: Código [${error.response?.status}]`,
    );
  }
}

export async function loadUserMachines(userid: string): Promise<Machine[]> {
  try {
    const response = await client.get<Machine[]>(
      `/busca_maquinas_usuario_idusuario/?id=${encodeURIComponent(userid)}`,
    );

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(
      `Erro ao carregar máquinas do usuário. Status code: ${response.status}`,
    );
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar informações: Código [${error.response?.status}]`,
    );
  }
}

export async function updateMachine(machine: Machine): Promise<Machine> {
  try {
    const response = await client.put<Machine>("/atualiza_maquina", machine);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(
      `Erro ao atualizar a máquina. Status code: ${response.status}`,
    );
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao atualizar a máquina: Código [${error.response?.status}]`,
    );
  }
}

export const loadMachineImages = async (
  machineId: string,
): Promise<string[]> => {
  try {
    console.log("machineId: ", machineId);
    const response = await client.post<string[]>(
      "/recupera_imagens_maquina",
      machineId,
      {
        headers: {
          "Content-Type": "application/json",
        },
      },
    );

    if (!response.data) {
      throw new Error("Erro ao carregar imagens da máquina");
    }

    const imagesPath = response.data;

    const imagesUrls: string[] = [];
    imagesPath.map((path) => {
      const imageUrl = `${import.meta.env.VITE_URL_BASE}${path}`;
      imagesUrls.push(imageUrl);
    });
    return imagesUrls;
  } catch (error) {
    console.error(error);
    throw error;
  }
};

export async function loadMachineRentValue(
  dados: MachineRentValue,
): Promise<number> {
  try {
    const response = await client.post<number>("/calcula_valor_aluguel", {
      idmaquina: dados.idmaquina,
      medida_prazo: dados.medida_prazo,
      prazo: dados.prazo,
    });

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(
      `Erro ao calcular o valor do aluguel da máquina. Status code: ${response.status}`,
    );
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao calcular o valor do aluguel da máquina: Código [${error.response?.status}]`,
    );
  }
}

export async function loadMachineById(id: string): Promise<Machine> {
  try {
    const response = await client.get<Machine>(
      `/busca_maquina_id/?id=${encodeURIComponent(id)}`,
    );

    if (response.status === 200) {
      const data = response.data;
      return data;
    } else {
      console.warn("Resposta inesperada:", response.status);
      throw new Error(
        `Erro ao carregar a máquina. Status code: ${response.status}`,
      );
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar informações da máquina: Código [${error.response?.status}]`,
    );
  }
}

export async function createMachine(
  machine: MachineInput,
): Promise<MachineIds> {
  try {
    const response = await client.post<MachineIds>(
      `/cadastra_maquina`,
      machine,
    );

    if (response.status === 200) {
      const data = response.data;
      return data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(
      `Erro ao cadastrar a máquina. Status code: ${response.status}`,
    );
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao cadastrar informações da máquina: Código [${error.response?.status}]`,
    );
  }
}
