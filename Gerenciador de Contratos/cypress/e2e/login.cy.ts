describe('Teste login', () => {

  beforeEach(() => {
    cy.visit('http://localhost:5173/');
    cy.login('user12@u.com', 'Senha1234.');
  });

  it('login realizado', () => {
   
    cy.get('.nav-left > :nth-child(2) > .class1').click();

    
    cy.url().should('eq', 'http://localhost:5173/');
  });

});

