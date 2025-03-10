import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription } from "@/components/ui/card";
import {
  loadMachinePublicId,
  loadUserMachines,
  loadMachineImages,
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
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";
import { Button } from "@/components/ui/button";
import { formatCurrency, formatDate } from "@/services/api/format/format";

export default function MachineDetails() {
  const navigate = useNavigate();
  const { publicid } = useParams();
  const [machine, setMachine] = useState<Maquina>();
  const [error, setError] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);
  const [machineOwner, setMachineOwner] = useState(false);

  

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

  const [images, setImages] = useState<string[]>();
  const [loadingImage, setLoadingImage] = useState(true);

  const fetchMachineImage = async (machineId: string) => {
    try {
      const response = await loadMachineImages(machineId);
      const imageUrl = response;
      console.log("imageUrl: ", imageUrl);
      setImages(imageUrl);
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
            <CardContent className="mt-4">
              <div className="machine-details-image">
                {loadingImage ? (
                  <div>
                    <Skeleton className="h-[35vh] w-[25vw] ml-10 rounded-xl" />
                  </div>
                ) : error ? (
                  <div className="image-placeholder">
                    Erro ao carregar a imagem
                  </div>
                ) : images ? (
                  <Carousel className="w-[80%] ml-10">
                  <CarouselContent>
                    {images.map((image) => (
                      <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel">
                      <img className="rounded-xl mb-4" src={image} alt={`Imagem de ${machine?.nome}`} />
                    </CarouselItem>
                    ))}
                  </CarouselContent>
                  <CarouselPrevious />
                  <CarouselNext />
                </Carousel>
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
                  {formatDate(machine?.dataatualizacao, +0)}
                </p>

                <div className="m-2">
                {machine?.disponivelaluguel === "Sim" ?
                (
                  <Button className="m-2"
                    onClick={() => {navigate(`/rent-machine/${machine.idpublico}`)}}
                  >Alugar máquina</Button>
                ) : (
                  <Button disabled={true}>Máquina indisponível para aluguel</Button>
                )}

                {machineOwner ? (
                  <>
                    <Button
                      className="m-2"
                      onClick={() => {
                        navigate(`/update-machine/${publicid}`);
                      }}
                    >
                      Atualizar máquina
                    </Button>
                  </>
                ) : (
                  <></>
                )}
                </div>
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
                  Você não está conectado!
                </AlertDialogTitle>
                <AlertDialogDescription style={{ color: "hsl(var(--text))" }}>
                  Você precisa estar logado pra fazer isso, por favor 
                  entre na sua conta.
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
      </main>
    </Layout>
  );
}
