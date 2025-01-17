import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { listMachine, loadMachineImage } from "@/services/api/machine/machine";
import { Button } from "@/components/ui/button";
import { Machine as Maquina} from "@/interfaces/machine";
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
} from "@/components/ui/alert-dialog"

export default function Machine() {
  const navigate = useNavigate();
  const [machines, setMachines] = useState<Maquina[]>([]);
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

  const verifyAccountStatus = () => {
    if(localStorage.getItem("USER_ID")){
      navigate('/logado');
      return;
    }
    toggleAlert();
    return;
  }

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
        <Card className="machine-card">
        <CardHeader>
        <CardTitle>
          MÁQUINA        
        </CardTitle>
        </CardHeader>
        <CardContent>  
        <div className="machine-image-home">
          {loadingImage ? (
            <div>
              <Skeleton className="h-[50vh] w-[30vw] rounded-xl" />
            </div>
          ) : error ? (
            <div className="image-placeholder">Erro ao carregar imagem</div>
          ) : image ? (
            <img src={image} alt={`Imagem de ${machine.nome}`} />
          ) : (
            <div className="image-placeholder">Imagem indisponível</div>
          )}
        </div>
        <h1>{machine.nome}</h1>    
        <p>Descrição: {machine.descricao}</p>
        <CardDescription className="quadro3">
        Preço: R$ {machine.valoraluguel}
        </CardDescription>
        <CardDescription>
        <Button onClick={() => {verifyAccountStatus()}}>Ver mais</Button>
        </CardDescription>
        <p>Categoria: {machine.categoria}</p>
        </CardContent>
      </Card>
    );
  }
  return (
    <Layout>
      <main>      
      <p 
        style={{color: 'hsl(var(--text))', fontSize: '25px'}}
        className="ml-[5%]"
        >Máquinas Disponíveis</p>  
        <div className="machine-grid">          
          {machines.length === 0 ? (
            <div>
              <p>Houve um erro ao carregar as máquinas. Reporte o problema aqui:</p>
              <Button>Relatar problema</Button>
            </div>
          ) : (
            machines.map((machine: Maquina) => (
              <div key={machine.idmaquina}>
              <MachineCard machine={machine} />
            </div>
            ))
          )}
        </div>           
        <div>            
          <Button onClick={()=> {navigate("/create-machine")}}>Ir para cadastro de máquinas</Button>
          <Button onClick={()=> {navigate("/user-profile")}}>Ir para o perfil</Button>

          <AlertDialog open={showAlert} onOpenChange={toggleAlert}>
          <AlertDialogTrigger asChild></AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
            <AlertDialogTitle style={{color: 'hsl(var(--text))'}}>UEPA! Pode não!</AlertDialogTitle>
            <AlertDialogDescription style={{color: 'hsl(var(--text))'}}>
              Você precisa estar logado pra fazer isso {">"}:( faz favor de entrar na sua conta.
            </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
            <AlertDialogCancel  style={{color: 'hsl(var(--text))', backgroundColor: 'rgb(136, 39, 39)'}}>Depois eu entro, tmj</AlertDialogCancel>
            <AlertDialogAction onClick={() => {navigate('/login')}}>Tabo, vou entrar :(</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
        </div> 
      </main>
    </Layout>
  );
}
