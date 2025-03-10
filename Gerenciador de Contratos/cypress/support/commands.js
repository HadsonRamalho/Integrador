Cypress.Commands.add('login', (email, senha) => {
    cy.get('.nav-right > :nth-child(2) > .class1').click()
    cy.get('[type="email"]').type('user12@u.com')
    cy.get('[type="password"]').type('Senha1234.')
    cy.get('.login > .inline-flex').click()
    cy.get('.nav-left > :nth-child(2) > .class1').click()



});

Cypress.Commands.add('realizarAluguelMaquina', () => {
    cy.get(':nth-child(1) > [data-radix-aspect-ratio-wrapper=""] > .rounded-xl > .p-6').first().click();
    cy.get('div.m-2 > .inline-flex').click();
  
    cy.get('[name="nome"]').type('Laura');
    cy.get('[name="email"]').type('user12@u.com');
    cy.get('[name="cpf"]').type('125.623.812-08');
    cy.get('[name="cep"]').type('39610000');
    cy.get('[name="pais"]').type('Brasil');
    cy.get('[name="estado"]').type('MG');
    cy.get('[name="cidade"]').type('Itinga');
    cy.get('[name="bairro"]').type('centro');
    cy.get('[name="endereco"]').type('av francisco');
    cy.get('[name="numero"]').type('123');
    cy.get('[name="complemento"]').type('casa');
  
    cy.get('.p-6.justify-center > .inline-flex').click();
  });

