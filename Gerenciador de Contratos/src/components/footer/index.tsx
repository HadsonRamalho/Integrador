import "./footer.css";

function Footer() {
  return (
    <footer className="footer-container" aria-label="Rodapé institucional">

      <section className="footer-section ">
        <h2 >Institucional</h2>
        <ul>
          <li> Sobre Nós</li>
          <li><a href="/home">Pagina Inicial</a></li>
          <li><a href="/HowWorks">Como Funciona</a></li>
          <li><a href="/Machine"> Maquinas</a></li>
        </ul>
      </section>

      <section className="footer-section">
        <h2 >Fale Conosco</h2>
        <ul>
          <li>e-mail</li>
          <li>perguntas recentes</li>
        </ul>
      </section>

    </footer>
  );
}

export default Footer;
