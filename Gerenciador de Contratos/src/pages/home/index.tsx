import Layout from "@/layouts/default";
import "@/components/home/home.css";
import { Button } from "@/components/ui/button";
import { Forklift, Handshake, Headset, MousePointerClickIcon, Search } from "lucide-react";
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import teste from "@/assets/teste.jpg";
import escavadeira from "@/assets/escavadeira.jpg";
import motoniveladora from "@/assets/motoniveladora.jpg";
import { Input } from "@/layouts";
import { Carousel, CarouselContent, CarouselItem } from "@/components/ui/carousel";

function Home() {
  return (
    <>
      <Layout>
        <main className="home-container">
          <section className="titulo-section">
            <h1>Locação de Maquinas e Equipamentos</h1>
            <p>Encontre a máquina ideal para sua obra ou disponibilize seus equipamentos para locação com segurança e sem burocracia.</p>
            <div className="busca-search">
              <Input placeholder="Buscar Equipamentos.." />
              <Button className=""><Search />Buscar</Button>

            </div>
          </section>

          <section className="beneficio-section">
            <h1>Por Que Alugar Na MAQEXPRESS?</h1>
            <div className="beneficios">

              <Card>
                <Forklift size={40} color="#29a366" />
                <CardTitle> Variedade</CardTitle>
                <CardContent>
                  Máquinas de diferentes categorias para atender sua necessidade, sempre disponíveis e prontas para uso.
                </CardContent>
              </Card>
              <Card>
                <MousePointerClickIcon size={40} color="#29a366" />
                <CardTitle> Facilidade</CardTitle>
                <CardContent>
                  Processo 100% online, sem burocracia! Alugue em poucos cliques e receba onde precisar.
                </CardContent>
              </Card>
              <Card>
                <Handshake size={40} color="#29a366" />
                <CardTitle>Segurança</CardTitle>
                <CardContent>
                  Garantimos que todos os usuários são verificados para evitar fraudes e proporcionar negociações seguras.
                  Formalize seu aluguel com contratos eletrônicos seguros, sem necessidade de papelada ou deslocamentos.
                  Utilizamos tecnologias de criptografia para proteger suas transações financeiras.
                  Proteja os equipamentos alugados contra danos ou imprevistos, garantindo mais tranquilidade na locação.
                </CardContent>
              </Card>
              <Card>
                <Headset size={40} color="#29a366" />
                <CardTitle>  Suporte Agilizado</CardTitle>
                <CardContent>
                  Nossa equipe está sempre pronta para ajudar, garantindo uma experiência ágil e sem complicações.
                </CardContent>
              </Card>
            </div>
          </section>

          <section className="destaque-section">
            <div className="destaque">
              <h1>Destaques Da Semana</h1>
              <Carousel>
                <CarouselContent className="-ml-2 md:-ml-4">
                  <CarouselItem className="pl-2 md:pl-4"><Card>
                    <img src={teste} alt="" />
                    <CardTitle>Roçadeira</CardTitle>
                    <CardContent>R$ 1000</CardContent>
                  </Card></CarouselItem>
                  <CarouselItem className="pl-2 md:pl-4">
                    <Card>
                      <img src={escavadeira} alt="" />
                      <CardTitle>Escavadeira</CardTitle>
                      <CardContent>R$ 2300</CardContent>
                    </Card></CarouselItem>
                  <CarouselItem className="pl-2 md:pl-4"><Card>
                    <img src={motoniveladora} alt="" />
                    <CardTitle>Motoniveladora</CardTitle>
                    <CardContent>R$ 120000</CardContent>
                  </Card></CarouselItem>
                </CarouselContent>
              </Carousel>



            </div>
          </section>
        </main>
      </Layout>
    </>
  );
}

export default Home;
