export interface ContractRequest{
   idsolicitacao: string;
   idlocador: string; 
   idlocatario: string;
   idmaquina: string;
   medidatempolocacao: string;
   origemsolicitacao: string;
   statussolicitacao: string;
   prazolocacao: number;
   valorsolicitacao: number;
   datasolicitacao: string;
}