import { Input } from "@/layouts";
import { useState } from "react";
import "./search.css";
import { Button } from "../ui/button";
import { Search } from "lucide-react";

const itens = ["Escavadeira", "Roçadeira", "Maquina", "Trator", "Motoniveladora"];

const SearchFilter = () => {
  const [search, setSearch] = useState("");
  const LowerSearch = search.toLowerCase();

  const filteredItems = itens.filter((item) =>
    item.toLowerCase().includes(LowerSearch)
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
          {filteredItems.map((item, index) => (
            <li key={index}>{item}</li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default SearchFilter;
