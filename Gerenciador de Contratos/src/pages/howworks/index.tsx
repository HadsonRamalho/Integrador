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
            <p >Lorem ipsum dolor sit amet consectetur adipisicing elit. Tenetur temporibus ullam neque! Reprehenderit atque, hic aspernatur natus ipsum similique enim et magnam accusantium. Aperiam commodi modi tempora libero voluptate est Lorem ipsum dolor sit amet consectetur, adipisicing elit. Nobis quasi nam minima, omnis, nulla ad odio ea voluptatibus tempore ratione quae accusantium modi, eligendi asperiores! Ducimus corrupti consequuntur illo ratione.
            </p>
        </div>
          <section>
            <div className="question-one">
              <div className="text">
                <h2>Como Alugar uma Máquina?</h2>
                
                <p>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Harum soluta facilis alias blanditiis nisi, officia cupiditate laborum atque repudiandae reiciendis molestiae quidem illum quod minus iste hic vitae. Quibusdam, repellendus.
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Facilis exercitationem velit ullam, odit optio maxime. Suscipit nobis alias esse. Facere neque sunt fugiat temporibus eos illum est eveniet repudiandae? Sapiente?
                Lorem ipsum dolor sit, amet consectetur adipisicing elit. Assumenda provident nam, molestias sed saepe ipsam enim nesciunt nemo laudantium, cum doloribus natus voluptatibus excepturi ducimus aspernatur harum aperiam. Libero, eius.
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quidem ratione veritatis molestiae labore incidunt, ipsum cupiditate. Nulla dolores harum ratione at? Quae, laboriosam ipsa. Aliquid ea non culpa reiciendis! Aliquam.
                </p>
              </div>
              <img src= {Dawn} alt="Trator estacionado em um campo"/>
            
            </div>
            
            <div className="question-two">
              <img src={Built} alt="Escavadeira amarela em uma colina"/>
                <div className="text">
                <h2><span style={{color: " #29A366"}}> MAQEXPRESS </span> é Confiavél?</h2>
                <p>Lorem ipsum dolor sit, amet consectetur adipisicing elit. Ullam obcaecati labore minus voluptate laudantium nihil sit ad tempora repellendus, sapiente, autem enim eum molestiae ipsam itaque sed necessitatibus cupiditate in!
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. Eos tempore eveniet deleniti explicabo quia dolore exercitationem tempora? Molestias, animi ullam. Voluptate quidem culpa aliquam vitae labore debitis magni illum quo.
                Lorem ipsum dolor, sit amet consectetur adipisicing elit. Numquam similique amet molestias rerum, impedit corrupti quam officia iusto praesentium temporibus necessitatibus. Magnam adipisci illum saepe et velit consequuntur dolorem dolor?
                </p>
                </div>
              </div>

            <div className="question-three" >
              <div className="text">
                <h2>E Se Eu Ainda Tiver Com Dúvida?</h2>
                <p>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus nemo expedita quidem voluptates. Nam quasi in saepe vel. Sit laudantium facere eos nobis eius in. Eligendi vitae omnis id ad.
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
