import { loadMachineImage } from "@/services/api/machine/machine";
import { AspectRatio } from "@radix-ui/react-aspect-ratio";
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
    <AspectRatio ratio={16 / 9}>
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
        <CardContent>
          <div className="machine-image-home">
            {loadingImage ? (
              <div>
                <Skeleton className="h-[30vh] w-[90%] rounded-xl" />
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