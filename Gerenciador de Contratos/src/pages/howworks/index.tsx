import Layout from "@/layouts/default";
import Dawn from "@/assets/maquina1.jpg";
import Built from "@/assets/maquina2.jpg";
import Besley from "@/assets/maquina3.jpg";
import "@/components/howworks/howworks.css";




export default function HowWorks() {
  return (
    <Layout>
      <main className="main-container">
        <div className= "headline" >
          <h1>
              Como Funciona o <span style={{color:" #29A366"}}> MAQEXPRESS </span>
            </h1>
            <p className="text-headline">Nosso sistema foi desenvolvido para conectar locadores e clientes de forma simples, rápida e segura. Com uma plataforma intuitiva, você pode encontrar a máquina ideal para seu projeto sem burocracia. Basta navegar pelas opções disponíveis, comparar os equipamentos e escolher aquele que melhor atende às suas necessidades.
            Além disso, garantimos um processo transparente, desde a solicitação até a finalização do aluguel, com contratos digitais e suporte dedicado para garantir a melhor experiência.
            </p>
        </div>
          <section>
            <div className="question-one">
              <div className="text">
                <h2>Como Alugar uma Máquina?</h2>
                
                <p>Alugar uma máquina pelo MAQEXPRESS é simples e rápido! Basta seguir estes passos:</p>

                <ol className="list">
                  <li><strong>Está cadastrado </strong> - Crie uma conta para começar a usar.</li>
                  <li><strong>Escolha a máquina</strong> - Navegue pela lista de equipamentos que  disponíveis e selecione o que melhor atende ás suas necessidades.</li>

                  <li><strong>Solicite o aluguel</strong> - Preencha seus dados, corretamente e envie a solicitação para o locator</li>

                  <li><strong>Aguarde a aprovação</strong> - O locador irá confirmar a disponibilidade da máquina e aceitar ou recusar sua solitação.</li>

                  <li><strong>Assine o contrato</strong> - Após a aprovação, um contrato digital será gerado automaticamente, garantindo a segurança de ambas as partes, com todos os termos e condições  definidos</li>

                  <li><strong>Retire a máquina
                  </strong> - Com o contrato assinado, você pode combinar a retirada da máquina com o locator, conforme o prazo estabelecido. A máquina estará pronta para uso!</li>
                </ol>
                
                
                
              </div>
              <img src= {Dawn} alt="Trator estacionado em um campo"/>
            
            </div>
            
            <div className="question-two">
              <img src={Built} alt="Escavadeira amarela em uma colina"/>
                <div className="text">
                <h2><span style={{color: " #29A366"}}> MAQEXPRESS </span> é Confiavél?</h2>
                <p className="text-two"> Sim! A <span style ={{color: "#29A366"}}> MAQEXPRESS</span> foi pensada e desenvolvida para garatir um processo de locação seguro, transparente e eficiente. Nosso sistema conta com medidas de proteção para ambas as partes,  assegurando que lacadores e clientes tenham uma experiência Confiavél.
                A MAQEXPRESS é a solução ideal para quem busca um aluguel de máquinas confiável
                </p>
                </div>
              </div>

            <div className="question-three" >
              <div className="text">
                <h2>E Se Eu Ainda Tiver Com Dúvida?</h2>
                <p >Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus nemo expedita quidem voluptates. Nam quasi in saepe vel. Sit laudantium facere eos nobis eius in. Eligendi vitae omnis id ad.
                Lorem ipsum dolor, sit amet consectetur adipisicing elit. Vitae quam enim tempora aspernatur excepturi! Consequuntur necessitatibus enim quisquam excepturi corporis expedita placeat, consequatur officia sint tempore nam tempora sapiente nostrum!
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Nesciunt a quo nulla ullam nihil! Temporibus at mollitia eos voluptates cupiditate. Placeat molestiae accusantium odio, quia possimus laboriosam amet animi voluptatem.
                </p>
              </div>
              <img src={Besley} alt="Escavadeira laranja"/>
   
            </div>

          </section>
      </main>
    </Layout>
  );
}
