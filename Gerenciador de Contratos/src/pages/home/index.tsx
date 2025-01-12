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

  useEffect(() => {
    const listMachines = async () => {
      const machines = await listMachine();
      console.log(machines);
      setMachines(machines);
    };
    listMachines();    
  }, []);

  return (
    <>
      <Layout>
        <main>
          <div className="home">            
          <h1>Locação De Maquinas e Equipamentos</h1>
          </div>
          <div className="grid">
          {machines.length === 0 ? (
              <div>
                <p>Houve um erro ao carregar as máquinas. Reporte o problema aqui:</p>
                <Button>Relatar problema</Button>
              </div>
            ) : (
              machines.map((machine: Machine) => (
                <div key={machine.idmaquina}>
                <Card className="quadro3">
                <CardHeader>
                  <CardTitle>
                  <h1>[Imagem da Máquina]</h1>
                  </CardTitle>
                </CardHeader>
                <CardContent>  
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
              </div>
              ))
            )}
            </div>
            <div>            
            <Button onClick={()=> {navigate("/logado")}}>Ir para a rota 'logado'</Button>
          </div>
          
        </main>
      </Layout>
    </>
  );
}

export default App;
