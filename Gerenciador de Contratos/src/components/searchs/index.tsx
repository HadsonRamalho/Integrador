import { Input } from "@/layouts";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "../ui/button";
import { Search } from "lucide-react";

const maquinas = [
  {id: 1, nome: "Escavadeira"},
  {id: 2, nome: "Roçadeira"},
  {id: 3, nome: "Maquina"},
  {id: 4, nome: "Trator"},
  {id: 5, nome: "Motoniveladora de Combate"},
];

const SearchFilter = () => {
  const [search, setSearch] = useState("");
  const navigate  = useNavigate();
  
  const PalavraBuscada = search.toLowerCase();
  
  const filteredItems = maquinas.filter((maquina) =>
    maquina.nome.toLowerCase().includes(PalavraBuscada)
  );
  

  return (
    <div className="busca-search">
      <Input
        type="text"
        value={search}
        placeholder="Buscar Equipamentos..."
        onChange={(e) => setSearch(e.target.value)}
        className="search-input"
      />
      <Button>
        <Search />
        Buscar
      </Button>


      {search && filteredItems.length > 0 && (
        <ul className="search-results">
          {filteredItems.map((maquina) => (
            <li
             key={maquina.id}
             onClick={() => navigate(`/maquinas/${encodeURIComponent(maquina.nome)}`)}
              >
             {maquina.nome}
            </li>
          ))}
        </ul>
      )}
      {filteredItems.length === 0 && search && (
        <p className="no-results">Nenhuma máquina encontrada.</p>
      )}
    </div>
  );
};

export default SearchFilter;
