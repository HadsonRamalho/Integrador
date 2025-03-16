import { User, UserId, UserInput } from "@/interfaces/user";
import { client } from "..";


export async function createUser(
  usuario: UserInput
): Promise<UserId> {
  try {
    const response = await client.post<UserId>("/cadastra_usuario", 
      usuario
    );

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao realizar cadastro. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao realizar cadastro:", error.response?.status, error.message);
    throw new Error(
      `Falha ao realizar cadastro: ${error.response?.status || "sem status"}`
    );
  }
}

export async function loginUser(
  email: string,
  password: string
): Promise<UserId> {
  try {
    const response = await client.post<UserId>("/realiza_login", {
      email,
      senha: password,
    });

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao realizar login. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao realizar login:", error.response?.status, error.message);
    throw new Error(
      `Falha ao realizar login: ${error.response?.status || "sem status"}`
    );
  }
}

export async function loadUserById(
  id: string
):Promise<User>{
  try {
    const url = `/busca_usuario_id/?id=${encodeURIComponent(id)}`;
    const response = await client.get<User>(url);

    if (response.status === 200) {
      return response.data;
    }
    console.warn("Resposta inesperada:", response.status);
    throw new Error(`Erro ao buscar o usuário. Status code: ${response.status}`);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao buscar o usuário:", error.response?.status, error.message);
    throw new Error(
      `Falha ao buscar o usuário: ${error.response?.status || "sem status"}`
    );
  }  
}