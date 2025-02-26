import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
} from "@/components/ui/card";
import { listMachine, loadMachineImage } from "@/services/api/machine/machine";
import { Button } from "@/components/ui/button";
import { Machine as Maquina } from "@/interfaces/machine";
import { useNavigate } from "react-router-dom";
import { Skeleton } from "@/components/ui/skeleton";
import "@/components/machine/machine.css";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { AspectRatio } from "@/components/ui/aspect-ratio";

export default function Machine() {
  const navigate = useNavigate();
  const [machines, setMachines] = useState<Maquina[]>([]);
  const [filter, setFilter] = useState("");

  const [error, setError] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);

  useEffect(() => {
    const listMachines = async () => {
      const machines = await listMachine();
      console.log(machines);
      setMachines(machines);
    };
    listMachines();
  }, []);

  const verifyAccountStatus = (id: string) => {
    if (localStorage.getItem("USER_ID")) {
      navigate(`/machine-details/${id}`);
      return;
    }
    toggleAlert();
    return;
  };

  const MachineCard: React.FC<{ machine: Maquina }> = ({ machine }) => {
    const [image, setImage] = useState("");
    const [loadingImage, setLoadingImage] = useState(true);

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
      fetchMachineImage(machine.idmaquina);
    }, [machine.idmaquina]);

    return (
      <AspectRatio ratio={16 / 9}>
        <Card
          className="machine-card  hover:cursor-pointer"
          onClick={() => {
            verifyAccountStatus(machine.idpublico);
          }}
        >
          <CardContent>
            <div className="machine-image-home">
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
                  alt={`Imagem de ${machine.nome}`}
                />
              ) : (
                <div className="image-placeholder">Imagem indisponível</div>
              )}
            </div>
            <h1>{machine.nome}</h1>
            <CardDescription>
              <p className="machine-card-description">
                Preço: <strong>R$ {machine.valoraluguel}</strong>
              </p>
              <p className="machine-card-description">
                Categoria: {machine.categoria}
              </p>
            </CardDescription>
          </CardContent>
        </Card>
      </AspectRatio>
    );
  };

  return (
    <Layout>
      <main className="mt-10">
        <div className="flex justify-center mb-4">
          <Card className="bg-[hsl(var(--machine-card-bg))] w-[50%] h-[120px] border-[hsl(var(--primary))]">
            <CardHeader>
              <p>Filtrar Máquinas</p>
            </CardHeader>
            <CardContent>
              <CardDescription className="mb-4">
              <div className="flex justify-center">
                <select
                  className="w-[60%] pl-2 bg-[hsl(var(--background))] h-[30px] text-[hsl(var(--text))] border-[hsl(var(--primary))] rounded-md border-[1px]"
                  onChange={(e) => setFilter(e.target.value)}
                  value={filter}
                  required
                >
                  <option value="">Todas as máquinas</option>
                  {machines
                    .map((machine) => machine.categoria)
                    .filter(
                      (categoria, index, self) =>
                        self.indexOf(categoria) === index
                    )
                    .map((categoria) => (
                      <option key={categoria} value={categoria}>
                        {categoria}
                      </option>
                    ))}
                </select>
              </div>
              </CardDescription>
            </CardContent>
          </Card>
        </div>
        <div className="machine-grid">
          {machines.length === 0 ? (
            <div>
              <p>
                Houve um erro ao carregar as máquinas. Reporte o problema aqui:
              </p>
              <Button>Relatar problema</Button>
            </div>
          ) : (
            machines
              .filter((machine) =>
                filter ? machine.categoria === filter : true
              )
              .map((machine: Maquina) => (
                <div
                  key={machine.idmaquina}
                  style={{ marginTop: "4vh", width: "90%", height: "768px" }}
                >
                  <MachineCard machine={machine} />
                </div>
              ))
          )}
        </div>
        <div>
          <AlertDialog open={showAlert} onOpenChange={toggleAlert}>
            <AlertDialogTrigger asChild></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle style={{ color: "hsl(var(--text))" }}>
                  UEPA! Pode não!
                </AlertDialogTitle>
                <AlertDialogDescription style={{ color: "hsl(var(--text))" }}>
                  Você precisa estar logado pra fazer isso {">"}:( faz favor de
                  entrar na sua conta.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel
                  style={{
                    color: "hsl(var(--text))",
                    backgroundColor: "rgb(136, 39, 39)",
                  }}
                >
                  Depois eu entro, tmj
                </AlertDialogCancel>
                <AlertDialogAction
                  onClick={() => {
                    navigate("/login");
                  }}
                >
                  Tabo, vou entrar :(
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </main>
    </Layout>
  );
}
