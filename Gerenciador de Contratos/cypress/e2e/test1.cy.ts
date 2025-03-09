describe('template spec', () => {
  it('passes', () => {
    cy.visit(' http://localhost:5173/',{ responseTimeout: 30000 })
    cy.get('.nav-right > :nth-child(2) > .class1').click()
    cy.get('.link-login').click()
    cy.get('[placeholder="Nome"]').type('David')
    cy.get('[type="email"]').type('david4356@gmai.com')
    cy.get('[placeholder="CPF/CNPJ"]').type('798.390.650-17')
    cy.get('[type="password"]').type('davidordad$65790')
    cy.get('.login > .inline-flex').click()
    cy.get('.link-login').click()
    cy.get('[type="email"]').type('david4356@gmai.com')
    cy.get('[type="password"]').type('davidordad$65790')
    cy.get('.login > .inline-flex').click()
    

    
  })
})