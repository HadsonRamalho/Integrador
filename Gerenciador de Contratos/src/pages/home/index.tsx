import Layout from "@/layouts/default";
import "@/components/home/home.css";
import { Button } from "@/components/ui/button";
import {
  Forklift,
  Handshake,
  Headset,
  MousePointerClickIcon,
  Search,
} from "lucide-react";
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import teste from "@/assets/teste.jpg";
import escavadeira1 from "@/assets/escava.jpg";
import escavadeira from "@/assets/escavadeira.jpg";
import caminhao from "@/assets/caminhao.jpg";
import motoniveladora from "@/assets/motoniveladora.jpg";
import moto_serra from "@/assets/motoserra.jpg";
import trator from "@/assets/trator.jpg";
import { Input } from "@/layouts";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";

function Home() {
  return (
    <>
      <Layout>
        <main className="home-container">
          <section className="titulo-section">
            <h1>Locação de Maquinas e Equipamentos</h1>
            <p>
              Encontre a máquina ideal para sua obra ou disponibilize seus
              equipamentos para locação com segurança e sem burocracia.
            </p>
            <div className="busca-search">
              <Input placeholder="Buscar Equipamentos.." />
              <Button className="">
                <Search />
                Buscar
              </Button>
            </div>
          </section>

          <section className="beneficio-section">
            <h1>Por Que Alugar Na MAQEXPRESS?</h1>
            <div className="beneficios">
              <Card>
                <Forklift size={40} color="#29a366" />
                <CardTitle> Variedade</CardTitle>
                <CardContent>
                  Máquinas de diferentes categorias para atender sua
                  necessidade, sempre disponíveis e prontas para uso.
                </CardContent>
              </Card>
              <Card>
                <MousePointerClickIcon size={40} color="#29a366" />
                <CardTitle> Facilidade</CardTitle>
                <CardContent>
                  Processo 100% online, sem burocracia! Alugue em poucos cliques
                  e receba onde precisar.
                </CardContent>
              </Card>
              <Card>
                <Handshake size={40} color="#29a366" />
                <CardTitle>Segurança</CardTitle>
                <CardContent>
                  Garantimos que todos os usuários são verificados para evitar
                  fraudes e proporcionar negociações seguras. Formalize seu
                  aluguel com contratos eletrônicos seguros, sem necessidade de
                  papelada ou deslocamentos. Utilizamos tecnologias de
                  criptografia para proteger suas transações financeiras.
                  Proteja os equipamentos alugados contra danos ou imprevistos,
                  garantindo mais tranquilidade na locação.
                </CardContent>
              </Card>
              <Card>
                <Headset size={40} color="#29a366" />
                <CardTitle> Suporte Agilizado</CardTitle>
                <CardContent>
                  Nossa equipe está sempre pronta para ajudar, garantindo uma
                  experiência ágil e sem complicações.
                </CardContent>
                
              </Card>
            </div>
          </section>

          <section className="teste">
          <h1>Encontre O Equipamento Ideal Para Sua Obra</h1>
            <Carousel>
              <CarouselContent>
                <CarouselItem  className="md:basis-1/2 lg:basis-1/3">
                  <img src={caminhao} alt=""/>
                  <CardTitle>Caçamba</CardTitle>
                </CarouselItem>
                <CarouselItem  className="md:basis-1/2 lg:basis-1/3">
                  <img src={moto_serra} alt=""/>
                  <CardTitle>Motosserra</CardTitle>
                </CarouselItem>
                <CarouselItem  className="md:basis-1/2 lg:basis-1/3">
                  <img src={escavadeira1} alt=""/>
                  <CardTitle>Escavadeira</CardTitle>
                </CarouselItem>
                <CarouselItem  className="md:basis-1/2 lg:basis-1/3">
                  <img src={trator} alt=""/>
                  <CardTitle>Trator</CardTitle>
                  </CarouselItem>
              </CarouselContent>
                  <CarouselPrevious />
                <CarouselNext />
            </Carousel>
          </section>

          <section className="destaque-section">
            <div className="destaque">
              <h1>Destaques Da Semana</h1>
              <Carousel>
                <CarouselContent>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3">
                    <img src={teste} alt="" />
                    <CardTitle>Roçadeira</CardTitle>
                    <CardContent>R$10092</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3">
                    <img src={escavadeira} alt="" />
                    <CardTitle>Escavadeira</CardTitle>
                    <CardContent>R$12292</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3">
                    <img src={motoniveladora} alt="" />
                    <CardTitle>Motoniveladora</CardTitle>
                    <CardContent>R$2540</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3">
                    <img src={escavadeira} alt="" />
                    <CardTitle>Escavadeira</CardTitle>
                    <CardContent>R$5410</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3">
                    <img src={motoniveladora} alt="" />
                    <CardTitle>Motoniveladora</CardTitle>
                    <CardContent>R$10000</CardContent>
                  </CarouselItem>
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
              </Carousel>
            </div>
          </section>
        </main>
      </Layout>
    </>
  );
}

export default Home;
