import { Machine } from "@/interfaces/machine";
import Layout from "@/layouts/default";
import {
  loadMachineImage,
  loadUserMachines,
} from "@/services/api/machine/machine";
import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
} from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import "@/components/machine-list/machine-list.css";
import { Machine as Maquina } from "@/interfaces/machine";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import MachineFilter from "@/components/machine-filter";

export const MachineList = () => {
  const MachineCard: React.FC<{ machine: Maquina }> = ({ machine }) => {
    const [image, setImage] = useState("");
    const [loadingImage, setLoadingImage] = useState(true);
    const [error, setError] = useState(false);
    const navigate = useNavigate();

    const fetchMachineImage = async (machineId: string) => {
      try {
        console.log("machineId: ", machineId);
        const response = await loadMachineImage(machineId);
        const imageUrl = response;
        setImage(imageUrl);
        setLoadingImage(false);
      } catch (error) {
        console.error(error);
        setError(true);
      } finally {
        setLoadingImage(false);
      }
    };

    useEffect(() => {
      if (machine) {
        fetchMachineImage(machine.idmaquina);
      }
    }, [machine]);

    return (
      <Card
        className={`machine-list-card ${
          machines.length === 1 ? "single-item" : ""
        } hover:cursor-pointer`}
        onClick={() => {
          navigate(`/machine-details/${machine?.idpublico}`);
        }}
      >
        <CardContent>
          <div className="machine-list-image">
            {loadingImage ? (
              <div>
                <Skeleton className="h-[30vh] w-[30vw] rounded-xl" />
              </div>
            ) : error ? (
              <div className="image-placeholder">Erro ao carregar imagem</div>
            ) : image ? (
              <img
                src={image}
                className="p-0 w-full"
                alt={`Imagem de ${machine?.nome}`}
              />
            ) : (
              <div className="image-placeholder">Imagem indisponível</div>
            )}
          </div>
          <h1>{machine?.nome}</h1>
          <CardDescription>
            <p className="machine-list-card-description">
              Preço: <strong>R$ {machine?.valoraluguel}</strong>
            </p>
            <p className="machine-list-card-description">
              Categoria: {machine?.categoria}
            </p>
          </CardDescription>
        </CardContent>
      </Card>
    );
  };

  const [machines, setMachines] = useState<Machine[]>([]);
  const [filter, setFilter] = useState("");

  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    const loadMachines = async (id: string) => {
      const machineArray = await loadUserMachines(id);
      console.log(machineArray || "Nenhuma máquina");
      setMachines(machineArray);
    };
    console.log(machines, id);
    if (machines.length === 0 && id) {
      loadMachines(id);
    }
  }, [machines]);

  const filteredMachines = machines.filter((machine) =>
    filter ? machine.categoria === filter : true
  );

  return (
    <Layout>
      <main className="mt-10 mb-10">
        <div className="flex justify-center items-center">
          <div className="rounded-md w-[50vw]">
            <MachineFilter
              machines={machines}
              filter={filter}
              setFilter={setFilter}
            />
          </div>
        </div>
        <Card className="bg-[hsl(var(--machine-card-bg))] m-4 border-[hsl(var(--primary))]">
            <CardHeader className="text-center">
            <p className="text-[1.2rem]">Minhas Máquinas</p>
            </CardHeader>
          <CardContent>
          <div className={`machine-list-grid ${filteredMachines.length === 1 ? 'single-item' : ''}`}>
          {filteredMachines.length === 0 ? (
            <Card>
              <CardHeader>
                <h2 className="text-[hsl(var(--primary))]">
                  Erro ao carregar lista de máquinas
                </h2>
              </CardHeader>
              <CardContent>
                <div>
                  <p className="mb-2 text-[hsl(var(--primary))]">
                    Você ainda não cadastrou uma máquina.
                    <br />
                  </p>
                  <div className="m-4">
                    <Button className="m-2">Relatar problema</Button>
                    <Button className="m-2">Cadastrar uma máquina</Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ) : (
            filteredMachines.map((machine: Maquina) => (
              <div
                key={machine.idmaquina}
                className={`machine-list-card ${filteredMachines.length === 1 ? 'single-item' : ''}`}
                style={{ marginTop: "4vh", width: "90%", height: "768px" }}
              >
                <MachineCard machine={machine} />
              </div>
            ))
          )}
          </div>
          </CardContent>
        </Card>
      </main>
    </Layout>
  );
};
