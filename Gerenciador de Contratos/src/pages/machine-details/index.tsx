import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription } from "@/components/ui/card";
import {
  loadMachinePublicId,
  loadUserMachines,
  loadMachineImages,
  loadMachineRentValue,
} from "@/services/api/machine/machine";
import { MachineRentValue, Machine as Maquina } from "@/interfaces/machine";
import { useNavigate, useParams } from "react-router-dom";
import { Skeleton } from "@/components/ui/skeleton";
import "@/components/machine-details/machine-details.css";
import { Input } from "@/layouts";
import { Label } from "@/components/ui/label";
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
  const [machineHour, setMachineHour] = useState<number>(0);
  const [machineDay, setMachineDay] = useState<number>(0);
  const [machineWeek, setMachineWeek] = useState<number>(0);
  

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

  const fetchMachinePrices = async (idmaquina: string) => {
    const dados_horas: MachineRentValue = {
      idmaquina: idmaquina,
      medida_prazo: 'Horas',
      prazo: 1
    };
    const dados_dias: MachineRentValue = {
      idmaquina: idmaquina,
      medida_prazo: 'Dias',
      prazo: 1
    };
    const dados_semanas: MachineRentValue = {
      idmaquina: idmaquina,
      medida_prazo: 'Semanas',
      prazo: 1
    };
    try {
      const resHour = await loadMachineRentValue(dados_horas);
      const resDay = await loadMachineRentValue(dados_dias);
      const resWeek = await loadMachineRentValue(dados_semanas);
      setMachineHour(resHour);
      setMachineDay(resDay);
      setMachineWeek(resWeek);
    } catch (error) {
      console.error(error); 
    }

  }

  useEffect(() => {
    if(machine){
      fetchMachinePrices(machine.idmaquina);
    }
  }, [machine]);

  return (
    <Layout>
      <main>
        <div className="flex justify-center items-center">
          <Card className="machine-details-card w-full md:w-[60%]">
            {machine ? (
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
              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Nome da Máquina</Label>
              <Input
                value={machine.nome}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

              <CardDescription className="m-2">
              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Valor do Aluguel (por mês)</Label>
              <Input
                value={formatCurrency(machine.valoraluguel)}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
              
              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Valor do Aluguel (por semana)</Label>
              <Input
                value={formatCurrency(machineWeek)}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
              
              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Valor do Aluguel (por dia)</Label>
              <Input
                value={formatCurrency(machineDay)}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
              
              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Valor do Aluguel (por hora)</Label>
              <Input
                value={formatCurrency(machineHour)}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
              

              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Categoria da Máquina</Label>
              <Input
                value={machine.categoria}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Número de Série</Label>
              <Input
                value={machine.numeroserie}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Disponível para Aluguel?</Label>
              <Input
                value={machine.disponivelaluguel}
                disabled={true}
                className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

              <Label className="text-[hsl(var(--text))] mt-2 mb-2">Descrição</Label>
              <textarea
                value={machine.descricao}
                disabled={true}
                className="p-2 h-20 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

                <div className="grid grid-cols-1 md:flex justify-center items-center m-2">
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
            ) : (<CardContent>
              <p className="text-[hsl(var(--text))]">Carregando a máquina...</p>
            </CardContent>)}
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
                  className="text-[hsl(var(--text)) bg-[#882727]"
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