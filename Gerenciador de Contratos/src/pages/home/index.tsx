import Layout from "@/layouts/default";
import  "@/components/home/home.css";
import { Button } from "@/components/ui/button";
import { Search } from "lucide-react";
import { Card, CardContent, CardTitle } from "@/components/ui/card";


function Home() {
  return (
    <>
      <Layout>
        <main>
          <section>
            <h1>Locação de Maquinas e Equipamentos</h1>
            <p>Encontre a máquina ideal para sua obra ou disponibilize seus equipamentos para locação com segurança e sem burocracia.</p>
            <div>
              <input placeholder="Buscar Equipamentos.." />
              <Button className=""><Search/>Buscar</Button>
              
            </div>
          </section>
          <section>
            <h1>Por Que Alugar Na MAQEXPRESS?</h1>
            <Card>
              <CardTitle>Variedade</CardTitle>
              <CardContent>
                Maquinas de diferentes categorias para atender sua necessidade
              </CardContent>
            </Card>
            <Card>
              <CardTitle>Facilidade</CardTitle>
              <CardContent>
                Processo 100% oninle, sem burocracia
              </CardContent>
            </Card>
            <Card>
              <CardTitle>Segurança</CardTitle>
              <CardContent>
               a
              </CardContent>
            </Card>

          </section>
        </main>
      </Layout>
    </>
  );
 }

export default Home;
