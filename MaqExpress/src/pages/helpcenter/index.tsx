import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/layouts";
import { Search } from "lucide-react";
import { useState } from "react";
import "@/components/helpcenter/helpcenter.css"
import Layout from "@/layouts/default";


export default function HelpCenter() {
  const [search, setSearch] = useState("");

  const duvidas = [
    { id: 1, categoria: "FAQ", descricao: "como faço isso?" },
    { id: 2, categoria: "Tutorial", descricao: "como faço pra atualizar meus dados?" },
    { id: 3, categoria: "Suporte", descricao: "Como entro em contato com suporte?" },
    { id: 4, categoria: "Como me cadastro?", descricao: "Acesse a página de cadastro" },
  ]
  const filteredDoubts = duvidas.filter((duvidas) =>
    duvidas.categoria.toLowerCase().includes(search.toLowerCase())
    ||
    duvidas.descricao.toLowerCase().includes(search.toLowerCase())
  );
  return (
    <Layout>
      <div >
        <h1 >Central de Ajuda</h1>
      <hr />
      <div className="search-busca mt-10 ml-10 mr-10">
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
            <Card key={duvidas.id} className="faq w-[90%] m-4">
              <CardContent>
                <p className="text-black text-[1.25rem] mt-2 mb-2">{duvidas.categoria}</p>
                <p className="text-black">{duvidas.descricao}</p>

              </CardContent>
            </Card>
          ))
        ) : (
          <p className="">Nenhum resultado encontrado.</p>
        )}
        <div className="flex items-center justify-center">
        <Button className="botao mb-4"><a href="mailto:gerenciadordecontratosgdc@gmail.com">Falar com o Suporte</a></Button>

        </div>
      </div>
    </div>
    </Layout>
  );
}