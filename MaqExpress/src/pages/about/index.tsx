import "@/components/about/about.css";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import Layout from "@/layouts/default";
import { useNavigate } from "react-router-dom";
export default function About() {
  const navigate = useNavigate();
  return (
    <Layout>
      <>
        <div>
          <section>
            <h2>Sobre a MaqExpress</h2>
            <div>
              <p>
                A <strong>MaqExpress</strong> é uma plataforma inovadora para o
                aluguel de máquinas e equipamentos, conectando mineradoras e
                locadoras de forma ágil e eficiente.
              </p>
              <p>
                Nosso objetivo é facilitar a contratação e gestão de
                equipamentos, eliminando burocracias e tornando o processo mais
                acessível e transparente para todos os envolvidos.
              </p>
              <p>
                Com a MaqExpress, empresas podem listar suas máquinas
                disponíveis, enquanto contratantes encontram exatamente o que
                precisam com segurança e rapidez.
              </p>
              <p>
                <strong>
                  Simples, rápido e confiável: esse é o jeito MaqExpress de
                  conectar negócios!
                </strong>
              </p>
            </div>
            <section>
              <Button>
                <a onClick={() => navigate("/")}>Conheça a MAQEXPRESS</a>
              </Button>
            </section>
          </section>

          <section className="teste1">
            <div className="teste2">
              <div className="teste3">
                <Card className="teste4">
                  <CardContent className="teste5">
                  <h2>Nosso Propósito</h2>
                  <p>
                    A <strong>MaqExpress</strong> nasceu para transformar o
                    setor de aluguel de máquinas e equipamentos, proporcionando
                    mais agilidade, acessibilidade e segurança para locadoras e
                    contratantes.
                  </p>
                  </CardContent>
                </Card>
                <Card className="teste4">
                  <h2>Nosso Objetivo</h2>
                  <p>
                    Nosso objetivo é simplificar a busca e a contratação de
                    equipamentos, eliminando burocracias e otimizando o processo
                    para que empresas possam focar no que realmente importa:
                    produtividade e crescimento.
                  </p>
                </Card>
                <Card className="teste4 text">
                  <h2>Nosso Compromisso</h2>
                  <p>
                    Estamos comprometidos em oferecer uma plataforma eficiente,
                    transparente e confiável, garantindo que locadores e
                    contratantes tenham uma experiência segura e satisfatória.
                  </p>
                </Card>
              </div>
            </div>
          </section>
        </div>
      </>
    </Layout>
  );
}
