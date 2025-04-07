import { Input } from "@/layouts";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Search } from "lucide-react";
import "@/components/searchs/searchs.css"
import { listMachine } from "@/services/api/machine/machine";
import { Machine } from "@/interfaces/machine";

const SearchFilter = () => {
  const [search, setSearch] = useState("");
  const navigate  = useNavigate();
  
  const PalavraBuscada = search.toLowerCase();

  const [maquinas, setMaquinas] = useState<Machine[]>([]);
  
  useEffect(() => {
    const listMachines = async () => {
      const maquinas = await listMachine();
      console.log(maquinas);
      setMaquinas(maquinas);
    };
    listMachines();
  }, []);
  
  const filteredItems = maquinas.filter((maquina) =>
    maquina.nome.toLowerCase().includes(PalavraBuscada.toLowerCase()) ||
    maquina.categoria.toLowerCase().includes(PalavraBuscada.toLowerCase())
  );
  

  return (
      <div className="container-search">
        <div className="search-busca">
        <Search  className="search-icon"/>
      <Input
        type="text"
        value={search}
        placeholder="Buscar Equipamentos..."
        onChange={(e) => setSearch(e.target.value)}
        className="search-input"
      />
      </div>


      {search && filteredItems.length > 0 && (
        <ul className="search-results">
          {filteredItems.map((maquina) => (
            <li
             key={maquina.idmaquina}
             className="search-item"
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
