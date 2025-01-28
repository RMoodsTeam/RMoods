describe('SidebarDrawer', () => {
  it('Checks if sidebar drawer can be opened in a tablet viewport', () => {
    const jwt = Cypress.env('RMOODS_JWT');
    Cypress.log(Cypress.env());
    if (!jwt) {
      cy.log('No JWT found, skipping');
      return;
    }

    // Mock user info response
    cy.intercept('GET', 'http://localhost:8001/api/user*', {
      statusCode: 200,
      body: {
        id: 'test-user-id',
        name: 'Test User',
        email: 'test@example.com',
        picture: 'https://lh3.googleusercontent.com/a-/AOh14Gg6s9c=s96-c',
      },
    }).as('getUserInfo');

    cy.setCookie('RMOODS_JWT', jwt);
    cy.visit('http://localhost:8000/#/dashboard');
    cy.viewport('ipad-2');

    cy.get('#sidebar-burger').click();
  });
});
