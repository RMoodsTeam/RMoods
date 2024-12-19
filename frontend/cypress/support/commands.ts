/// <reference types="cypress" />

declare namespace Cypress {
  interface Chainable<Subject = any> {
    visitPageAndScreenshotIt(page: string): Chainable<Element>;
  }
}

Cypress.Commands.add('visitPageAndScreenshotIt', (page: string) => {
  let pageName = page === '' ? 'Home' : page;
  pageName = pageName.charAt(0).toUpperCase() + pageName.slice(1);

  // Cypress automatically adds an env if it is prefixed with CYPRESS_ and when you use it, it strips the prefix
  // JWT token for this was created on 19th Dec 2024 and is valid for 300 days
  const jwt = Cypress.env('RMOODS_JWT');
  if (!jwt) {
    cy.log('No JWT found, skipping');
    return;
  }

  cy.setCookie('RMOODS_JWT', jwt);

  cy.visit('http://localhost:8000/#/' + page);
  if (page !== 'login') {
    cy.location('hash').should('not.include', 'login');
  }
  cy.get('#main-button').should('be.visible');
  cy.percySnapshot(`${pageName} Page`);
});
