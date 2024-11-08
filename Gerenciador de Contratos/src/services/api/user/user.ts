import { User } from "@/interfaces/user";
import { client } from "..";

export async function loginUser(
  email: string,
  password: string
): Promise<User> {
  const { data } = await client.post<User>("/login", {
    email,
    password,
  });
  return data;
}

