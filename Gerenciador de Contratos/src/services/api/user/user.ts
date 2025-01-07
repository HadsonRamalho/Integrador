import { UserId } from "@/interfaces/user";
import { client } from "..";

export async function loginUser(
  email: string,
  password: string
): Promise<UserId> {
  const { data } = await client.post<UserId>("/realiza_login", {
    email,
    senha: password,
  });
  return data;
}