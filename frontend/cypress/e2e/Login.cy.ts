describe('Login', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000/#/login');
  });

  it('Displays a title', () => {
    cy.get('h1').should('have.text', 'Welcome to RMoods!');
  });
});
