import Layout from "@/layouts/default";
import "@/components/home/home.css";
import {
  Forklift,
  Handshake,
  Headset,
  MousePointerClickIcon,

} from "lucide-react";
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import teste from "@/assets/teste.jpg";
import escavadeira1 from "@/assets/escava.jpg";
import escavadeira from "@/assets/escavadeira.jpg";
import cacamba from "@/assets/caminhao.jpg";
import motoniveladora from "@/assets/motoniveladora.jpg";
import moto_serra from "@/assets/motoserra.jpg";
import trator from "@/assets/trator.jpg";

import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";
import { useNavigate } from "react-router-dom";
import SearchFilter from "@/components/searchs";


function Home() {
  const navigate = useNavigate();


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
            <SearchFilter/>
          </section>

          <section className="beneficio-section">
            <h1>Por Que Alugar Na MAQEXPRESS?</h1>
            <div className="beneficios">
              <Card className="card">
                <Forklift size={40} color="#29a366" />
                <CardTitle> Variedade</CardTitle>
                <CardContent className="card-content">
                  Máquinas de diferentes categorias para atender sua
                  necessidade, sempre disponíveis e prontas para uso.
                </CardContent>
              </Card>
              <Card className="card">
                <MousePointerClickIcon size={40} color="#29a366" />
                <CardTitle> Facilidade</CardTitle>
                <CardContent className="card-content">
                  Processo 100% online, sem burocracia! Alugue em poucos cliques
                  e receba onde precisar.
                </CardContent>
              </Card>
              <Card className="card">
                <Handshake size={40} color="#29a366" />
                <CardTitle>Segurança</CardTitle>
                <CardContent className="card-content">
                  Garantimos que todos os usuários são verificados para evitar
                  fraudes e proporcionar negociações seguras. Utilizamos
                  contratos eletrônicos e criptografia para sua proteção.
                </CardContent>
              </Card>
              <Card className="card">
                <Headset size={40} color="#29a366" />
                <CardTitle> Suporte Agilizado</CardTitle>
                <CardContent className="card-content">
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
                <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel">
                  <img
                    className="hover:cursor-pointer"
                    src={cacamba}
                    alt="caçamba"
                    onClick={() => navigate(`/maquinas/${encodeURIComponent('caçamba')}`)}
                  />
                  <CardTitle>Caçamba</CardTitle>
                </CarouselItem>
                <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel">
                  <img
                    className="hover:cursor-pointer"
                    src={moto_serra}
                    alt="motosserra"
                    onClick={() => navigate(`/maquinas/${encodeURIComponent('motosserra')}`)}
                  />
                  <CardTitle>Motosserra</CardTitle>
                </CarouselItem>
                <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel">
                  <img
                    className="hover:cursor-pointer"
                    src={escavadeira1}
                    alt="escavadeira"
                    onClick={() => navigate(`/maquinas/${encodeURIComponent('escavadeira')}`)}
                  />
                  <CardTitle>Escavadeira</CardTitle>
                </CarouselItem>
                <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel">
                  <img
                    className="hover:cursor-pointer"
                    src={trator}
                    alt="trator"
                    onClick={() => navigate(`/maquinas/${encodeURIComponent('trator')}`)}
                  />
                  <CardTitle>Trator</CardTitle>
                </CarouselItem>
              </CarouselContent>
              <CarouselPrevious />
              <CarouselNext />
            </Carousel>
          </section>

          <section className="destaque-section">
            <div>
              <h1>Destaques Da Semana</h1>
              <Carousel>
                <CarouselContent>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel-item">
                    <img
                      className="hover:cursor-pointer"
                      src={teste}
                      alt=""
                      onClick={() => navigate(`/maquinas/${encodeURIComponent('Roçadeira')}`)}
                    />
                    <CardTitle>Roçadeira</CardTitle>
                    <CardContent>R$10092</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel-item">
                    <img
                      className="hover:cursor-pointer"
                      src={escavadeira}
                      alt=""
                      onClick={() => navigate(`/maquinas/${encodeURIComponent('Escavadeira')}`)}
                    />
                    <CardTitle>Escavadeira</CardTitle>
                    <CardContent>R$12292</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel-item">
                    <img
                      className="hover:cursor-pointer"
                      src={motoniveladora}
                      alt=""
                      onClick={() => navigate(`/maquinas/${encodeURIComponent('Motoniveladora')}`)}
                    />
                    <CardTitle>Motoniveladora</CardTitle>
                    <CardContent>R$2540</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel-item">
                    <img
                      className="hover:cursor-pointer"
                      src={escavadeira}
                      alt=""
                      onClick={() => navigate(`/maquinas/${encodeURIComponent('escavadeira')}`)}
                    />
                    <CardTitle>Escavadeira</CardTitle>
                    <CardContent>R$5410</CardContent>
                  </CarouselItem>
                  <CarouselItem className="md:basis-1/2 lg:basis-1/3 carousel-item">
                    <img
                      className="hover:cursor-pointer"
                      src={motoniveladora}
                      alt=""
                      onClick={() => navigate(`/maquinas/${encodeURIComponent('motoniveladora')}`)}
                    />
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
