import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Input } from "@/layouts"
import { useState } from "react"


export function DropdownMenuDemo() {
  const [cep, setCep] = useState("");
  const [pais, setPais] = useState("Brasil");
  const [estado, setEstado] = useState("");
  const [cidade, setCidade] = useState("");
  const [TriggerText, setTriggerText] = useState("Cep");
  
  
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
    <DropdownMenuLabel>Selecione </DropdownMenuLabel>
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
