export interface ContractPDF {
  idcontrato: string;

  nomelocatario: string;
  documentolocatario: string;
  tipodocumentolocatario: string;

  nomelocador: string;
  documentolocador: string;
  tipodocumentolocador: string;

  estadoenderecolocatario: string;
  cidadeenderecolocatario: string;
  cependerecolocatario: string;
  bairroenderecolocatario: string;
  logradouroenderecolocatario: string;
  numeroenderecolocatario: string;
  complementoenderecolocatario: string;

  estadoenderecolocador: string;
  cidadeenderecolocador: string;
  cependerecolocador: string;
  bairroenderecolocador: string;
  logradouroenderecolocador: string;
  numeroenderecolocador: string;
  complementoenderecolocador: string;

  estadoenderecoretirada: string;
  cidadeenderecoretirada: string;
  cependerecoretirada: string;
  bairroenderecoretirada: string;
  logradouroenderecoretirada: string;
  numeroenderecoretirada: string;
  complementoenderecoretirada: string;

  nomemaquina: string;
  numeroseriemaquina: string;
  valoraluguelmaquina: number;

  numerocontabanco: string;
  numeroagenciabanco: string;
  nomebanco: string;

  medidatempolocacao: string;

  prazolocacao: number;
  valorlocacao: number;

  datacontrato: string;
}

export interface Contract {
  idcontrato: string;
  idlocatario: string;
  idlocador: string;
  idenderecolocatario: string;
  idenderecolocador: string;
  idenderecoretirada: string;
  idmaquina: string;
  idsolicitacaocontrato: string;
  idcontabancarialocador: string;
  medidatempolocacao: string;
  cidadeforo: string;
  statuscontrato: string;
  prazolocacao: number;
  valorlocacao: number;
  datacontrato: string;
}