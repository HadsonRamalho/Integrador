import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Address } from "@/interfaces/address";
import { Input } from "@/layouts";
import { loadAddressUserId } from "@/services/api/address/address";
import { useEffect, useState } from "react";

export function DropdownMenuDemo({ triggerColor }: { triggerColor: string }) {
  const [cep, setCep] = useState("");
  const [pais, setPais] = useState("Brasil");
  const [estado, setEstado] = useState("");
  const [cidade, setCidade] = useState("");
  const TriggerText = "Sua Cidade";
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
    if (id) {
      buscaEndereco(id);
    }
    if (endereco) {
      setCep(endereco.cep);
      setCidade(endereco.cidade);
      setEstado(endereco.estado);
      setPais(endereco.pais);
      localStorage.setItem("cidade_dropdownmenu", endereco.cidade);
    }
  }, [endereco]);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger >
        {localStorage.getItem("cidade_dropdownmenu") || TriggerText}
      </DropdownMenuTrigger>
      <DropdownMenuContent style={{ zIndex: 1001 }}>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>
          <Input
            placeholder="CEP"
            type="cep"
            value={cep}
            onChange={(e) => setCep(e.target.value)}
            disabled={true}
          />
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>
          <Input
            placeholder="País"
            value={pais}
            onChange={(e) => setPais(e.target.value)}
            disabled={true}
          />
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>
          <Input
            placeholder="Estado"
            value={estado}
            onChange={(e) => setEstado(e.target.value)}
            disabled={true}
          />
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>
          <Input
            placeholder="Cidade"
            value={cidade}
            onChange={(e) => setCidade(e.target.value)}
            disabled={true}
          />
        </DropdownMenuLabel>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
