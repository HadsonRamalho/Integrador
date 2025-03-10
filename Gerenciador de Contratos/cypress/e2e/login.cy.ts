describe('Teste login', () => {

  beforeEach(() => {
    cy.visit('http://localhost:5173/');
    cy.login('user12@u.com', 'Senha1234.');
  });

  it('login realizado', () => {
   
  });

});
