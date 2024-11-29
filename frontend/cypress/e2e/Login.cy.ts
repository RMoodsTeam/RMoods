describe('Login', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000/RMoods/#/login');
  });

  it('Displays a title', () => {
    cy.get('h1').should('have.text', 'Welcome to RMoods!');
  });
});
