import { Notification } from "@/interfaces/notifications";
import { client } from "..";

export async function loadNotificationsByUserId(id: string): Promise<Notification[]> {
  try {
    const response = await client.get<Notification[]>(`/busca_notificacoes_idusuario/?id=${encodeURIComponent(id)}`);

    if (response.status === 200) {
      const data = response.data;
      return data;
    }else {
      console.warn("Resposta inesperada:", response.status);
      throw new Error(`Erro ao carregar as notificações. Status code: ${response.status}`);
    }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar informações das notificações: Código [${error.response?.status}]`
    );
  }
}