import { Machine } from "@/interfaces/machine";
import Layout from "@/layouts/default";
import {
  loadMachineImage,
  loadUserMachines,
} from "@/services/api/machine/machine";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import "@/components/machine-list/machine-list.css";
import { Machine as Maquina } from "@/interfaces/machine";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";

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
      className="machine-list-card  hover:cursor-pointer"
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

export const MachineList = () => {
  const [machines, setMachines] = useState<Machine[]>([]);

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

  return (
    <Layout>
      <main className="mt-10 mb-10">
        <div className="machine-list-grid">
          <div>
            {machines.length === 0 ? (
              <div>
                <p>
                  Houve um erro ao carregar as máquinas. Reporte o problema
                  aqui:
                </p>
                <Button>Relatar problema</Button>
              </div>
            ) : (
              machines.map((machine: Maquina) => (
                <div
                  key={machine.idmaquina}
                  style={{ width: "40%", height: "768px" }}
                >
                  <MachineCard machine={machine} />
                </div>
              ))
            )}
          </div>
        </div>
      </main>
    </Layout>
  );
};
