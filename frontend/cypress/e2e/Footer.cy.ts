describe('Footer', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000/#/');
  });
  it('Checks footer github link', () => {
    cy.get('#footer-github').should(
      'have.attr',
      'href',
      'https://github.com/RMoodsTeam/RMoods'
    );
  });
});
