import Layout from "@/layouts/default";
import Dawn from "@/assets/maquina1.jpg";
import Built from "@/assets/maquina2.jpg";
import Besley from "@/assets/maquina3.jpg";



export default function About() {
  return (
    <Layout>
      <main>
      <h1>
          Como Funciona o <span style={{color:" #29A366"}}> MAQEXPRESS </span>;
        </h1>
        <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Tenetur temporibus ullam neque! Reprehenderit atque, hic aspernatur natus ipsum similique enim et magnam accusantium. Aperiam commodi modi tempora libero voluptate est Lorem ipsum dolor sit amet consectetur, adipisicing elit. Nobis quasi nam minima, omnis, nulla ad odio ea voluptatibus tempore ratione quae accusantium modi, eligendi asperiores! Ducimus corrupti consequuntur illo ratione.
        </p>

        <div>
          <h2>Como Alugar uma Máquina?</h2>
          <p>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Harum soluta facilis alias blanditiis nisi, officia cupiditate laborum atque repudiandae reiciendis molestiae quidem illum quod minus iste hic vitae. Quibusdam, repellendus.</p>

          <img src= {Dawn}/>
          <p>Foto por <a href="https://unsplash.com/pt-br/fotografias/um-trator-esta-estacionado-em-um-campo-de-grama-alta-nJHPGuCDXHA">Dawn McDonald</a> no Unsplash</p>

        </div>
        
        <div>
        

        <img src={Built}/>

        <p>Foto por <a href="https://unsplash.com/pt-br/fotografias/uma-escavadeira-amarela-em-uma-colina-com-um-ceu-azul-no-fundo-zmW-UG2OX_M">Built Robotics</a> no Unsplash</p>

        <h2><span style={{color: " #29A366"}}> MAQEXPRESS </span> é Confiavél?;</h2>
        <p>Lorem ipsum dolor sit, amet consectetur adipisicing elit. Ullam obcaecati labore minus voluptate laudantium nihil sit ad tempora repellendus, sapiente, autem enim eum molestiae ipsam itaque sed necessitatibus cupiditate in!</p>
        </div>

        <div>
          <h2>E Se Eu Ainda Tiver Com Dúvida?</h2>
          <p>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus nemo expedita quidem voluptates. Nam quasi in saepe vel. Sit laudantium facere eos nobis eius in. Eligendi vitae omnis id ad.</p>

          <img src={Besley}/>
          <p>Foto por <a href="https://unsplash.com/pt-br/fotografias/escavadeira-laranja-k5l-zbRSPds">Luke Besley</a> no Unsplash</p>
        </div>

        <div>
          <h3>Dúvidas Frequentes</h3>

          <ol>
            <li>Pergunta 1?</li>
            <li>Pergunta 2?</li>
          </ol>  
             
        </div>

      </main>
    </Layout>
  );
}
