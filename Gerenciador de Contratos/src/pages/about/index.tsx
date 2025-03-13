import "@/components/about/about.css";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
export default function About() {
    const navigate = useNavigate();
  return (
  <>
  <div className="a">
  <section className="about-maqexpress">
      <h2>Sobre a MaqExpress</h2>
      <p>
        A <strong>MaqExpress</strong> é uma plataforma inovadora para o aluguel de máquinas e equipamentos,
        conectando mineradoras e locadoras de forma ágil e eficiente.
      </p>
      <p>
        Nosso objetivo é facilitar a contratação e gestão de equipamentos, eliminando burocracias
        e tornando o processo mais acessível e transparente para todos os envolvidos.
      </p>
      <p>
        Com a MaqExpress, empresas podem listar suas máquinas disponíveis, enquanto contratantes
        encontram exatamente o que precisam com segurança e rapidez.
      </p>
      <p>
        <strong>Simples, rápido e confiável: esse é o jeito MaqExpress de conectar negócios!</strong>
      </p>
      
    </section>
   
      <section className="purpose-personality">
      <h2>Nosso Propósito</h2>
      <p>
        A <strong>MaqExpress</strong> nasceu para transformar o setor de aluguel de máquinas e equipamentos, 
        proporcionando mais agilidade, acessibilidade e segurança para locadoras e contratantes.
      </p>
      <p>
        Nosso objetivo é simplificar a busca e a contratação de equipamentos, 
        eliminando burocracias e otimizando o processo para que empresas possam focar no que realmente importa: produtividade e crescimento.
      </p>
      </section>
      <section className="mt-4  purpose-personality">
      <h2>Nossa Personalidade</h2>
      <p>
        Somos inovadores, confiáveis e eficientes. Nossa plataforma foi criada para conectar 
        empresas de forma intuitiva e transparente, garantindo negociações seguras e satisfatórias.
      </p>
      <p>
        Acreditamos no trabalho em equipe, na tecnologia e na confiança mútua como pilares 
        fundamentais para um mercado de aluguel mais eficiente e acessível.
      </p>
    </section>
    <section>
    <Button className="mt-4"><a onClick={() => navigate("/")}>Conheça a MAQEXPRESS</a>
    </Button>
      </section>
    </div>
      </>
  
  );
}
