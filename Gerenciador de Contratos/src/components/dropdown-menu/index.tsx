import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Address } from "@/interfaces/address";
import { Input } from "@/layouts"
import { loadAddressUserId } from "@/services/api/address/address";
import { useEffect, useState } from "react"


export function DropdownMenuDemo() {
  const [cep, setCep] = useState("");
  const [pais, setPais] = useState("Brasil");
  const [estado, setEstado] = useState("");
  const [cidade, setCidade] = useState("");
  const [TriggerText, setTriggerText] = useState("Cep");
  const [endereco, setEndereco] = useState<Address>();
  
  useEffect(() => {
    async function buscaEndereco(id: string) {
      const res = await loadAddressUserId(id);
      if (!res) {
        console.log("Erro ao buscar endereço do usuário.");
        alert("Usuário não possui endereço");
      }
      const endereco = res;
      setEndereco(endereco);
    }
    const id = localStorage.getItem("USER_ID");
    if (id){
      buscaEndereco(id);
    }
    if (endereco){
      setCep(endereco.cep);
      setCidade(endereco.cidade);
      setEstado(endereco.estado);
      setPais(endereco.pais);
      localStorage.setItem("cidade_dropdownmenu", endereco.cidade);
    }
  }, [endereco]);
  
  const CarregaEndereco = async () =>{
    try {
      const res = await fetch(`https://viacep.com.br/ws/${cep}/json/`,
     {method:'GET'} );
     if(!res.ok){
      throw new Error( await res.text());
     }
     const endereco = await res.json();
     setEstado(endereco.estado);
     setCidade(endereco.localidade);
     console.log(endereco);
     setTriggerText(`${endereco.localidade}`);
     localStorage.setItem("cidade_dropdownmenu", endereco.localidade);
     
    } catch (error) {
      console.error(error);
    }
  }
  return (
    <DropdownMenu>
  <DropdownMenuTrigger>{localStorage.getItem("cidade_dropdownmenu")||TriggerText }</DropdownMenuTrigger>
  <DropdownMenuContent>
    <DropdownMenuLabel>Selecione</DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
     <Input
     placeholder="CEP"
     type="cep"
     value={cep}
     onChange={(e) =>setCep(e.target.value)}
     onBlur={CarregaEndereco}
     />
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="País"
      value={pais}
      onChange={(e)=> setPais(e.target.value)}
      />
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="Estado"
      value={estado}
      onChange={(e)=> setEstado(e.target.value)}
      />
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="Cidade"
      value={cidade}
      onChange={(e)=>setCidade(e.target.value)}
      />
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuItem>
    <Button onClick={CarregaEndereco}>Confirmar</Button>
    </DropdownMenuItem>
  </DropdownMenuContent>
</DropdownMenu>

  )
}
