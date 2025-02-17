import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription} from "@/components/ui/card";
import { loadMachinePublicId, loadMachineImage } from "@/services/api/machine/machine";
import { Machine as Maquina} from "@/interfaces/machine";
import { useNavigate, useParams } from "react-router-dom";
import { Skeleton } from "@/components/ui/skeleton";
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


export default function MachineDetails() {
  const navigate = useNavigate();
  const {publicid} = useParams();
  const [machine, setMachine] = useState<Maquina>();
  const [error, setError] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);

  useEffect(() => {
    const listMachines = async () => {
      if(publicid){
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

  useEffect(() => {
    if (machine){
        fetchMachineImage(machine.idmaquina);
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
            <div className="image-placeholder">Erro ao carregar a imagem</div>
          ) : image ? (
              <img src={image} alt={`Imagem de ${machine?.nome}`} />
          ) : (
            <div className="image-placeholder">Imagem indisponível</div>
          )}
        </div>
        <h1>{machine?.nome}</h1>    
        <CardDescription>
        <p className="machine-card-description">Preço: <strong>R$ {machine?.valoraluguel}</strong></p>
        <p className="machine-card-description">Categoria: {machine?.categoria}</p>
        </CardDescription>
        </CardContent>
      </Card>   

        </div>           
        <div>            
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
            <AlertDialogCancel onClick={() => {navigate('/')}}  style={{color: 'hsl(var(--text))', backgroundColor: 'rgb(136, 39, 39)'}}>Depois eu entro, tmj</AlertDialogCancel>
            <AlertDialogAction onClick={() => {navigate('/login')}}>Tabo, vou entrar :(</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
        </div> 
      </main>
    </Layout>
  );
}
