/// <reference types="cypress" />

declare namespace Cypress {
  interface Chainable<Subject = any> {
    visitPageAndScreenshotIt(page: string): Chainable<Element>;
  }
}

Cypress.Commands.add('visitPageAndScreenshotIt', (page: string) => {
  let pageName = page === '' ? 'Home' : page;
  pageName = pageName.charAt(0).toUpperCase() + pageName.slice(1);

  cy.visit('http://localhost:8000/#/' + page, {
    onBeforeLoad(win) {
      win.localStorage.setItem('RMOODS_JWT', 'TEST_TOKEN');
    },
  });
  cy.get('#main-button').should('be.visible');
  cy.percySnapshot(`${pageName} Page`);
});
