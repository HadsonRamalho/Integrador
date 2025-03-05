describe('template spec', () => {
  it('passes', () => {
    cy.visit(' http://localhost:5173/',{ responseTimeout: 30000 })
    cy.get(':nth-child(6) > .class1').click()
    cy.get('span.link-login > .link-login').click() 
    cy.get('[placeholder="Nome"]').type('David') 
    cy.get('[type="email"]').type('davidcadastro123@gmail.com')  
    cy.get('[placeholder="CPF/CNPJ"]').type('570.991.860-19')
    cy.get('[type="password"]').type('davidconta12345')
    cy.get('.login > .inline-flex').click()
    cy.get('[type="email"]').type('davidcadastro123@gmail.com')
    cy.get('[type="password"]').type('davidconta12345')
    cy.get('.button',{ responseTimeout: 30000 } ).click()
    
  })
})