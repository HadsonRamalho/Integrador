export interface Address{
  idendereco: string;
  pais: string;
  estado: string;
  cidade: string;
  cep: string;
  bairro: string;
  logradouro: string;
  numero: string;
  complemento: string;
}

export interface CreateUserAddress{
  idusuario: string;
  pais: string;
  estado: string;
  cidade: string;
  cep: string;
  bairro: string;
  logradouro: string;
  numero: string;
  complemento: string;
}