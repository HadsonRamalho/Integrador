export interface Machine {
    idmaquina: string;
    idpublico: string;
    nome: string;
    numeroserie: string;
    categoria: string;
    valoraluguel: number;
    disponivelaluguel: string;
    status: string;
    datacadastro: string; // ISO 8601 ("2023-01-01T12:00:00")
    dataatualizacao: string; // ISO 8601 ("2023-01-01T12:00:00")
    descricao: string;
}

export interface MachineRentValue{
    idmaquina: string;
    medida_prazo: string;
    prazo: number;
}