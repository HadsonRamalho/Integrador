import Layout from "@/layouts/default";
import  "@/components/navbar/navbar.css";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
function App() {
  const navigate = useNavigate();
  return (
    <>
      <Layout>
        <main>
          <div className="home">
            
          <h1>Locação De Maquinas e Equipamentos</h1>
          </div>
          <div className="grid">
            <div>
            <Card className="quadro3">
              <CardHeader>
                <CardTitle>
                <h1>[Imagem da Máquina]</h1>
                </CardTitle>
              </CardHeader>
              <CardContent>  
                <h1>Nome da Máquina</h1>    
                <CardDescription className="quadro3">
                  Preço: R$ 169,99
                </CardDescription>
                <CardDescription>
                  <Button onClick={()=> {navigate("/logado")}}>Ver mais</Button>
                </CardDescription>
              </CardContent>
            </Card>
            </div>
            <Button onClick={()=> {navigate("/logado")}}>Ir para a rota 'logado'</Button>
          </div>
          
        </main>
      </Layout>
    </>
  );
}

export default App;
