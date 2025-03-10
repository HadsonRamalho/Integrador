describe('Monitoramento da carregamento', () => {

  beforeEach(() => {
    cy.visit('http://localhost:5173/');
    cy.login('user12@u.com', 'Senha1234.');
  });

  it('mede tempo da pagina inicial ate a segunda', () => {
    const pagina1 = Date.now();
    cy.get(':nth-child(3) > .class1').click();
    cy.window().its('performance').invoke('mark', 'pagina2');
    cy.window().its('performance').invoke('measure', 'tempoPag1Pag2', 'navigationStart', 'pagina2');

    cy.window().then((win) => {
      const measure = win.performance.getEntriesByName('tempoPag1Pag2')[0];
      cy.log(`Tempo carregamento Pag1 -> Pag2: ${measure.duration}ms`);
    });
  });

  it('mede tempo da segunda para terceira', () => {
    cy.get(':nth-child(3) > .class1').click(); 

    cy.get(':nth-child(4) > .class1').click();
    cy.window().its('performance').invoke('mark', 'pagina3');
    cy.window().its('performance').invoke('measure', 'tempoPag2Pag3', 'navigationStart', 'pagina3');

    cy.window().then((win) => {
      const measure = win.performance.getEntriesByName('tempoPag2Pag3')[0];
      cy.log(`Tempo carregamento Pag2 -> Pag3: ${measure.duration}ms`);
    });
  });

  it('mede tempo da terceira para quarta', () => {
    cy.get(':nth-child(3) > .class1').click(); 
    cy.get(':nth-child(4) > .class1').click(); 

    cy.get(':nth-child(5) > .class1').click(); 
    cy.window().its('performance').invoke('mark', 'pagina4');
    cy.window().its('performance').invoke('measure', 'tempoPag3Pag4', 'navigationStart', 'pagina4');

    cy.window().then((win) => {
      const measure = win.performance.getEntriesByName('tempoPag3Pag4')[0];
      cy.log(`Tempo carregamento Pag3 -> Pag4: ${measure.duration}ms`);
    });
  });

});
