describe('HomeSnapshot', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000/#/');
  });

  it('Creates home snapshot', () => {
    cy.percySnapshot('Home');
  });
});
