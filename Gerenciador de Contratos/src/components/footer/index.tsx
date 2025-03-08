import "./footer.css";

function Footer() {
  return (
    <footer className="footer-container" aria-label="Rodapé institucional">
      <h1 className=" footer-section uppercase">MaqExpress</h1>

      <section className="footer-section ">
        <h2 className="uppercase" >Institucional</h2>
        <ul>
          <li> Sobre Nós</li>
          <li><a href="/">Pagina Inicial</a></li>
          <li><a href="/HowWorks">Como Funciona</a></li>
          <li><a href="/Machine"> Maquinas</a></li>
        </ul>
      </section>

      <section className="footer-section">
        <h2 className="uppercase">Fale Conosco</h2>
        <ul>
          <strong>E-mail</strong>
          <li>exemp@gmail.com</li>
          <li>Perguntas Frequentes</li>
          <strong>Contato</strong>
          <li>(33)0000-0000</li>
        </ul>
      </section>
    
    </footer>
  );
}

export default Footer;
