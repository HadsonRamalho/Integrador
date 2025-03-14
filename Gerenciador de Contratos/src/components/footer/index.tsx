import "./footer.css";

function Footer() {
  return (
    <footer className="footer-container" aria-label="Rodapé institucional">
      <h1 className=" footer-section uppercase">MaqExpress</h1>

      <section className="footer-section ">
        <h2 className="uppercase" >Institucional</h2>
        <ul>
          <li> Sobre nós</li>
          <li><a href="/">Página inicial</a></li>
          <li><a href="/HowWorks">Como funciona</a></li>
          <li><a href="/Machine"> Máquinas</a></li>
        </ul>
      </section>

      <section className="footer-section">
        <h2 className="uppercase">Fale conosco</h2>
        <ul>
          <strong>E-mail</strong>
          <li>gerenciadordecontratosgdc@gmail.com</li>
          <strong>Contato</strong>
          <li>(33)0000-0000</li>
        </ul>
      </section>
    
    </footer>
  );
}

export default Footer;
