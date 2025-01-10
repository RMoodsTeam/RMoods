describe('Theme Switch', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000/#/');
  });

  it('Should change theme when selecting different options', () => {
    cy.get('#theme-switch').find('button').click();

    cy.contains('button', 'Light').click();

    // cy.get('#theme-switch').contains('Light').click();
    cy.get('body')
      .should('have.css', 'background-color')
      .and('eq', 'rgb(255, 255, 255)');
    cy.get('#theme-switch').find('button').click();

    cy.contains('button', 'Dark').click();

    cy.get('body')
      .should('have.css', 'background-color')
      .and('eq', 'rgb(36, 38, 46)');
  });
});
