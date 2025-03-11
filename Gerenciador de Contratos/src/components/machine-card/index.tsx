import { loadMachineImage } from "@/services/api/machine/machine";
import { useEffect, useState } from "react";
import { Card } from "../ui/card";
import { useNavigate } from "react-router-dom";
import { Machine } from "@/interfaces/machine";
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
import {
  CardContent,
  CardDescription,
} from "@/components/ui/card";
import { Skeleton } from "../ui/skeleton";
export const MachineCard: React.FC<{ machine: Machine }> = ({ machine }) => {
  const [image, setImage] = useState("");
  const [loadingImage, setLoadingImage] = useState(true);
  const [error, setError] = useState(false);
  const navigate = useNavigate();
  
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);

  const fetchMachineImage = async (machineId: string) => {
    try {
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

  const verifyAccountStatus = (id: string) => {
    if (localStorage.getItem("USER_ID")) {
      navigate(`/machine-details/${id}`);
      return;
    }
    toggleAlert();
    return;
  };

  useEffect(() => {
    fetchMachineImage(machine.idmaquina);
  }, [machine.idmaquina]);

  return (
      <Card
        className="machine-card  hover:cursor-pointer"
        onClick={() => {
          verifyAccountStatus(machine.idpublico);
        }}
      >
        <div>
          <AlertDialog open={showAlert} onOpenChange={toggleAlert}>
            <AlertDialogTrigger asChild></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle style={{ color: "hsl(var(--text))" }}>
                  Você não está conectado!
                </AlertDialogTitle>
                <AlertDialogDescription style={{ color: "hsl(var(--text))" }}>
                Você precisa estar logado pra fazer isso, por favor 
                entre na sua conta.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel
                  className="text-[hsl(var(--text))] bg-[#882727]"
                >
                  Cancelar
                </AlertDialogCancel>
                <AlertDialogAction
                  onClick={() => {
                    navigate("/login");
                  }}
                >
                  Entrar/Registrar 
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
        <CardContent>
          <div className="machine-image-container">
            {loadingImage ? (
              <div>
                <Skeleton className="h-[30vh] w-[90%] rounded-xl" />
               
              </div>
            ) : error ? (
              <div className="image-placeholder">Erro ao carregar imagem</div>
            ) : image ? (
                <img
                src={image}
                alt={`Imagem de ${machine.nome}`}
              />
            ) : (
              <div className="image-placeholder">Imagem indisponível</div>
            )}
          </div>
          <div>
            <h1 className="ml-0 w-[250px]">{machine.nome}</h1>
          </div>
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
  );
};