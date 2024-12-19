/// <reference types="cypress" />

declare namespace Cypress {
  interface Chainable<Subject = any> {
    visitPageAndScreenshotIt(page: string): Chainable<Element>;
  }
}

Cypress.Commands.add('visitPageAndScreenshotIt', (page: string) => {
  let pageName = page === '' ? 'Home' : page;
  pageName = pageName.charAt(0).toUpperCase() + pageName.slice(1);
  cy.setCookie('RMOODS_JWT', Cypress.env('RMOODS_JWT'));

  cy.visit('http://localhost:8000/#/' + page);
  if (page !== 'login') {
    cy.location('hash').should('not.include', 'login');
  }
  cy.get('#main-button').should('be.visible');
  cy.percySnapshot(`${pageName} Page`);
});
