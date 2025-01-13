import Layout from "@/layouts/default";
import  "@/components/navbar/navbar.css";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import { listMachine } from "@/services/api/machine/machine";
import { Machine } from "@/interfaces/machine";
function App() {
  const navigate = useNavigate();
  const [machines, setMachines] = useState<Machine[]>([]);
  const [error, setError] = useState(false);

  useEffect(() => {
    const listMachines = async () => {
      const machines = await listMachine();
      console.log(machines);
      setMachines(machines);
    };
    listMachines();    
  }, []);

  const MachineCard = ({machine}) => {
    const [image, setImage] = useState("");
    const [loadingImage, setLoadingImage] = useState(true);

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const fetchMachineImage = async (machineId: any) => {
      try {
        console.log("machineId: ", machineId);
        const response = await fetch(`https://g6v9psc0-3003.brs.devtunnels.ms/recupera_imagem_maquina`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(machineId), 
        });
    
        if (!response.ok) {
          throw new Error('Erro ao carregar imagem da máquina');
        }
    
        const imagePath = await response.json();
        const imageUrl = `https://g6v9psc0-3003.brs.devtunnels.ms${imagePath}`;
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
        <Card className="quadro3">
        <CardHeader>
        <CardTitle>
          MÁQUINA        
        </CardTitle>
        </CardHeader>
        <CardContent>  
        <div className="machine-image-home">
          {loadingImage ? (
            <p>Carregando imagem...</p>
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
        <Button onClick={()=> {navigate("/logado")}}>Ver mais</Button>
        </CardDescription>
        <p>Categoria: {machine.categoria}</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <>
      <Layout>
        <main>
          <div className="grid">
          {machines.length === 0 ? (
              <div>
                <p>Houve um erro ao carregar as máquinas. Reporte o problema aqui:</p>
                <Button>Relatar problema</Button>
              </div>
            ) : (
              machines.map((machine: Machine) => (
                <div key={machine.idmaquina}>
                <MachineCard machine={machine} />
              </div>
              ))
            )}
            </div>
            <div>            
            <Button onClick={()=> {navigate("/create-machine")}}>Ir para cadastro de máquinas</Button>
            <Button onClick={()=> {navigate("/logado")}}>Ir para a rota 'logado'</Button>
          </div>
          
        </main>
      </Layout>
    </>
  );
 }

export default App;
