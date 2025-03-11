import Layout from "@/layouts/default";
import Dawn from "@/assets/maquina1.jpg";
import Built from "@/assets/maquina2.jpg";
import Besley from "@/assets/maquina3.jpg";
import "@/components/howworks/howworks.css";
import { Contact, MailCheck, BookCopy } from 'lucide-react';


export default function HowWorks() {
  return (
    <Layout>
      <main className="main-container">
        <div className= "headline" >
          <h1>
              Como funciona o <span style={{color:" #29A366"}}> MAQEXPRESS </span>
            </h1>
            <p className="text-headline">Nosso sistema foi desenvolvido para conectar locadores 
            e clientes de forma simples, rápida e segura. Com uma plataforma intuitiva, você pode
            encontrar a máquina ideal para seu projeto sem burocracia. Basta navegar pelas opções 
            disponíveis, comparar os equipamentos e escolher aquele que melhor atende às suas
            necessidades.
            Além disso, garantimos um processo transparente, desde a solicitação até a finalização
            do aluguel, com contratos digitais e suporte dedicado para garantir a melhor experiência.
            </p>
        </div>
          <section>
            <div className="question-one">
              <div className="text">
                <h2>Como alugar uma máquina?</h2>
                
                <p>Alugar uma máquina pelo MAQEXPRESS é simples e rápido! Basta seguir estes passos:</p>

                <ol className="list">
                  <li><strong>Se cadastre </strong> - Crie uma conta para começar a usar nosso sistema.</li>
                  <li><strong>Escolha a máquina</strong> - Navegue pela lista de equipamentos disponíveis e selecione o que melhor atende às suas necessidades.</li>

                  <li><strong>Solicite o aluguel</strong> - Preencha seus dados corretamente e envie a solicitação para o locador.</li>

                  <li><strong>Aguarde a aprovação</strong> - O locador irá confirmar a disponibilidade da máquina e aceitar sua solitação.</li>

                  <li><strong>Assine o contrato</strong> - Após a aprovação, um contrato digital será gerado automaticamente, garantindo a segurança de ambas as partes, com todos os termos e condições  definidos.</li>

                  <li><strong>Retire a máquina
                  </strong> - Com o contrato assinado, você pode combinar a retirada da máquina com o locador, conforme o prazo estabelecido.</li>
                </ol>
                
              </div>
              <img className="howworks-image"  src= {Dawn} alt="Trator estacionado em um campo"/>
            
            </div>
            
            <div className="question-two">
              <img className="howworks-image" src={Built} alt="Escavadeira amarela em uma colina"/>
                <div className="text">
                <h2><span style={{color: " #29A366"}}> MAQEXPRESS </span> é confiavél?</h2>
                <p className="text-two"> Sim! A <span style ={{color: "#29A366"}}> MAQEXPRESS</span> foi pensada e desenvolvida para garantir um processo de locação seguro, transparente e eficiente. Nosso sistema conta com medidas de proteção para ambas as partes,  assegurando que lacadores e clientes tenham uma experiência Confiavél.
                A MAQEXPRESS é a solução ideal para quem busca um aluguel de máquinas seguro.
                </p>
                </div>
              </div>

            <div className="question-three" >
              <div className="text-three">
                <h2>E se eu ainda estiver com dúvida?</h2>
                <p >Se ainda restarem dúvidas sobre o funcionamento ou o processo de aluguel, estamos aqui para ajudar!</p>

                <ul className="list-two">
                  <li>
                    <span className="icon"><MailCheck /></span>
                    Fale com o suporte - Nossa aquipe está disponível para responder suas perguntas e oferecer suporte durante todo o processo.
                  </li>
                  <li>
                    <span className="icon"><Contact/></span>
                    Entre em contato com o locador – Você pode conversar diretamente com o locador para esclarecer detalhes sobre a máquina ou a locação.</li>
                  <li>
                    <span className="icon"><BookCopy /></span>
                    Confira a seção de perguntas frequentes (FAQ) – Reunimos as dúvidas mais comuns para facilitar sua experiência.</li>
                </ul>

                <p>Não hesite em nos chamar! Estamos prontos para tornar sua experiência mais tranquila e segura. </p>

              </div>
              <img className="howworks-image"  src={Besley} alt="Escavadeira laranja"/>
   
            </div>

          </section>
      </main>
    </Layout>
  );
}
