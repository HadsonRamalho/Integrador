import { Button } from "@/components/ui/button";
import { Input } from "@/layouts";
import { Search } from "lucide-react";
import { useState } from "react";

export default function HelpCenter() {
      const [search, setSearch] = useState("");
    return (
        <>
        <div>
      <h1 >Central de Ajuda</h1>
      <div >
        <Search  />
        <Input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Pesquise por uma dúvida..."
          
        />
      </div>
        <p >Nenhum resultado encontrado.</p>
      <div >
        <Button >Falar com o Suporte</Button>
      </div>
    </div>
        </>
    );

}