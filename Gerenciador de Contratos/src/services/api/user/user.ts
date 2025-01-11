import { UserId } from "@/interfaces/user";
import { client } from "..";

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
    } else {
      console.warn("Resposta inesperada:", response.status);
      throw new Error(`Erro ao realizar login. Status code: ${response.status}`);
    }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error("Erro ao realizar login:", error.response?.status, error.message);
    throw new Error(
      `Falha ao realizar login: ${error.response?.status || "sem status"}`
    );
  }
}
