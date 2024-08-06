describe('Theme Switch', () => {
  beforeEach(() => {
    cy.visit('localhost:3000/', {
      onBeforeLoad (win) {
        cy.stub(win, 'matchMedia')
        .withArgs('(prefers-color-scheme: dark)')
        .returns({
          matches: false,
        })
      },
    })
  });

  it('Exists', () => {
    cy.get('#theme-switch').should('exist');
  });

  it('Detects users theme and switches it when button is clicked', () => {
    cy.get('#theme-switch').click().should(() => {
      expect(localStorage.getItem("theme")).to.eq("dark");
    })
  })
})
