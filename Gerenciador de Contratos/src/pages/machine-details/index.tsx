import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription } from "@/components/ui/card";
import {
  loadMachinePublicId,
  loadMachineImage,
  loadUserMachines,
} from "@/services/api/machine/machine";
import { Machine as Maquina } from "@/interfaces/machine";
import { useNavigate, useParams } from "react-router-dom";
import { Skeleton } from "@/components/ui/skeleton";
import "@/components/machine-details/machine-details.css";
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
import { Button } from "@/components/ui/button";

export default function MachineDetails() {
  const navigate = useNavigate();
  const { publicid } = useParams();
  const [machine, setMachine] = useState<Maquina>();
  const [error, setError] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);
  const [machineOwner, setMachineOwner] = useState(false);

  const formatDate = (dateparam: string | undefined) => {
    if (!dateparam) return "";
    console.log(dateparam);

    const date = new Date(dateparam);

    const brtOffset = -3 * 60;
    const localDate = new Date(date.getTime() + brtOffset * 60 * 1000);

    const hours = String(localDate.getHours()).padStart(2, "0");
    const minutes = String(localDate.getMinutes()).padStart(2, "0");
    const day = String(localDate.getDate()).padStart(2, "0");
    const month = String(localDate.getMonth() + 1).padStart(2, "0");
    const year = localDate.getFullYear();

    return `${hours}:${minutes} ${day}/${month}/${year}`;
  };

  const formatCurrency = (value: number | bigint) => {
    return new Intl.NumberFormat("pt-BR", {
      style: "currency",
      currency: "BRL",
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(value);
  };

  useEffect(() => {
    const listMachines = async () => {
      if (publicid) {
        const machine = await loadMachinePublicId(publicid);
        console.log(machine);
        setMachine(machine);
      }
    };
    listMachines();
  }, [publicid]);

  const [image, setImage] = useState("");
  const [loadingImage, setLoadingImage] = useState(true);

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

  const verifyUserMachines = async (userId: string, machineId: string) => {
    try {
      const response = await loadUserMachines(userId);
      const userMachines = response;
      const machine = userMachines.find(
        (machine) => machine.idmaquina === machineId
      );
      if (machine) {
        return true;
      }
    } catch (error) {
      console.error(error);
      return false;
    }
  };

  useEffect(() => {
    if (machine) {
      fetchMachineImage(machine.idmaquina);
    }
  }, [machine]);

  useEffect(() => {
    const userId = localStorage.getItem("USER_ID");
    if (userId && machine) {      
      verifyUserMachines(userId, machine.idmaquina).then((result) => {
        if (result) {
          setMachineOwner(true);
        }
      });
    }
  }, [machine]);

  return (
    <Layout>
      <main>
        <div>
          <Card className="machine-details-card">
            <CardContent>
              <div className="machine-details-image">
                {loadingImage ? (
                  <div>
                    <Skeleton className="h-[30vh] w-[30vw] rounded-xl" />
                  </div>
                ) : error ? (
                  <div className="image-placeholder">
                    Erro ao carregar a imagem
                  </div>
                ) : image ? (
                  <img src={image} alt={`Imagem de ${machine?.nome}`} />
                ) : (
                  <div className="image-placeholder">Imagem indisponível</div>
                )}
              </div>
              <p className="machine-details-name">{machine?.nome}</p>
              <CardDescription className="m-2">
                <p className="machine-card-description">
                  Preço:{" "}
                  <strong>{formatCurrency(machine?.valoraluguel || 0)}</strong>
                </p>
                <p className="machine-card-description">
                  Categoria: {machine?.categoria}
                </p>

                <p className="machine-card-description">
                  Número de série: {machine?.numeroserie}
                </p>
                <p className="machine-card-description">
                  Disponível para aluguel: {machine?.disponivelaluguel}
                </p>
                <p className="machine-card-description">
                  Status do anúncio: {machine?.status}
                </p>

                <p className="machine-card-description">
                  Descrição: {machine?.descricao}
                </p>

                <p className="machine-card-description">
                  Data de cadastro da máquina:{" "}
                  {formatDate(machine?.datacadastro)}
                </p>
                <p className="machine-card-description">
                  Data de atualização da máquina:{" "}
                  {formatDate(machine?.dataatualizacao)}
                </p>

                {machineOwner ? (
                  <>
                    <Button>Editar máquina</Button>
                  </>
                ) : (
                  <></>
                )}
              </CardDescription>
            </CardContent>
          </Card>
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
                  onClick={() => {
                    navigate("/");
                  }}
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
