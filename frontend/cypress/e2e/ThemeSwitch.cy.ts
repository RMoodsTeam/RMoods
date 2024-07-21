describe('Theme switch logic', () => {
  beforeEach(() => {
    console.log("Before each!");
    cy.visit('localhost:3000/', {
      onBeforeLoad (win) {
        cy.stub(win, 'matchMedia')
        .withArgs('(prefers-color-scheme: dark)')
        .returns({
          matches: false,
        })
      },
    })
  })
  it('Sees button', () => {
    cy.get('#theme-switch')
  })

  it('Detects users theme and switches it when button is clicked', () => {
    console.log("before click!")
    cy.get('#theme-switch').click().should(() => {
      expect(localStorage.getItem("theme")).to.eq("dark");
    })
  })
})