import "./footer.css";

function Footer() {
  return (
    <footer className="footer-container" aria-label="Rodapé institucional">
      <h1 className=" footer-section uppercase">MaqExpress</h1>

      <section className="footer-section ">
        <h2 className="uppercase" >Institucional</h2>
        <ul>
          <li><a href="/About">Sobre nós</a> </li>
          <li><a href="/">Página inicial</a></li>
          <li><a href="/HowWorks">Como funciona</a></li>
          <li><a href="/Machine"> Máquinas</a></li>
        </ul>
      </section>

      <section className="footer-section">
        <h2 className="uppercase">Fale conosco</h2>
        <ul>
          <strong>E-mail</strong>
          <li>
            <a href="mailto:gerenciadordecontratosgdc@gmail.com">gerenciadordecontratosgdc@gmail.com
            </a>
            </li>
          
          <strong>Contato</strong>
          <li>(33)1234-5678</li>
        </ul>
      </section>
    
    </footer>
  );
}

export default Footer;
