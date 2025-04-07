import axios from "axios";

export function handleAxiosError(error: unknown): void {
  if (!axios.isAxiosError(error)) {
    console.error("Erro desconhecido:", error);
    return;
  }

  if (error.response) {
    switch (error.response.status) {
      case 400:
        console.error("Erro 400: Requisição inválida. Verifique os dados enviados.", error.response.data);
        return;
      case 401:
        console.error("Erro 401: Não autorizado. Credenciais inválidas.", error.response.data);
        return;
      case 403:
        console.error("Erro 403: Proibido. Você não tem permissão para acessar este recurso.", error.response.data);
        return;
      case 404:
        console.error("Erro 404: Recurso não encontrado.", error.response.data);
        return;
      case 500:
        console.error("Erro 500: Erro interno do servidor. Possível inconsistência nos dados enviados ou problema no servidor.", error.response.data);
        return;
      default:
        console.error(`Erro ${error.response.status}: Ocorreu um erro inesperado.`, error.response.data);
        return;
    }
  }

  if (error.request) {
    console.error("Sem resposta do servidor. Verifique sua conexão com a internet ou tente novamente mais tarde.", error.request);
    return;
  }

  console.error("Erro inesperado ao fazer a requisição:", error.message);
}
