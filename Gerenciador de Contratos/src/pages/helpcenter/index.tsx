import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/layouts";
import { Search } from "lucide-react";
import { useState } from "react";
import "@/components/helpcenter/helpcenter.css"

export default function HelpCenter() {
      const [search, setSearch] = useState("");

const duvidas =[
  {id: 1, categoria :"FAQ", descricao:"como faço isso?"},
  {id: 2, categoria: "Tutorial", descricao: "como faço pra atualizar meus dados?"},
  {id: 3, categoria: "Suporte", descricao:"Como entro em contato com suporte?"},
]
  const filteredDoubts = duvidas.filter((duvidas) =>
    duvidas.categoria.toLowerCase().includes(search.toLowerCase())
  );
    return (

        <div>
      <h1 >Central de Ajuda</h1>
      <div  className="search-busca mt-10 ml-10 mr-10">
        <Search className="search-icon" />
        <Input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Pesquise por uma dúvida..."
          className="search-input"
          
        />
      </div>
      
      <div>

       {filteredDoubts.length > 0 ? (
        filteredDoubts.map((duvidas) => (
          <Card key={duvidas.id} className="faq">
            <CardContent>
              <h2 className="text-black">{duvidas.categoria}</h2>
              <h4 className="text-black">{duvidas.descricao}</h4>
            
            </CardContent>
          </Card>
        ))
      ) : (
        <p className="">Nenhum resultado encontrado.</p>
      )}
        <Button  className="botao">Falar com o Suporte</Button>
      </div>
      </div>
   
  
    );

}