import { useEffect, useState } from "react";
import { Machine as Maquina } from "@/interfaces/machine";
import { listMachine } from "@/services/api/machine/machine";
import { MachineCard } from "@/components/machine-card";
import { Button } from "@/components/ui/button";
import { Input } from "@/layouts";
import { useParams } from "react-router-dom";
import Layout from "@/layouts/default";
import "@/components/searchs/searchs.css";
import { Search } from "lucide-react";
const DetalhesMaquina = () => {
  const [maquinas, setMaquinas] = useState<Maquina[]>([]);
  const { busca } = useParams();
  const [filter, setFilter] = useState(busca);

  useEffect(() => {
    const listMachines = async () => {
      const maquinas = await listMachine();
      console.log(maquinas);
      setMaquinas(maquinas);
    };
    listMachines();
  }, []);

  const filteredMachines = maquinas.filter((maquina) =>
    maquina.nome.toLowerCase().includes(filter.toLowerCase())
  );

  return (
    <Layout>
      <main className="max-w-6xl mx-auto p-6">
        <div className="mt-10 mb-10 ">
          <div className="search-busca">
            <Input
              type="text"
              value={filter}
              onChange={(e) => setFilter(e.target.value)}
              placeholder="Buscar máquina..."
              className="search-input-result"
            />
            <Search className="search-icon" />
          </div>

          <div className="machine-grid">
            {maquinas.length === 0 ? (
              <div>
                <p>
                  Houve um erro ao carregar as máquinas. Reporte o problema
                  aqui:
                </p>
                <Button>Relatar problema</Button>
              </div>
            ) : filteredMachines.length > 0 ? (
              filteredMachines.map((maquina) => (
                <div
                  key={maquina.idmaquina}
                  style={{ marginTop: "4vh", width: "90%", height: "600px" }}
                >
                  <MachineCard machine={maquina} />
                </div>
              ))
            ) : (
              <p>Nenhuma máquina encontrada.</p>
            )}
          </div>
        </div>
      </main>
    </Layout>
  );
};

export default DetalhesMaquina;
