// Whenever a new page is created, we can add a new test to take a snapshot of the page
describe('Visual Regression tests', () => {
  it('Creates "/" snapshot', () => {
    cy.visit('http://localhost:8000/#/');
    cy.percySnapshot('Home');
  });

  it('Creates "/about" snapshot', () => {
    cy.visit('http://localhost:8000/#/about');
    cy.percySnapshot('About');
  });

  it('Creates "/dashboard" snapshot', () => {
    cy.visit('http://localhost:8000/#/dashboard');
    cy.percySnapshot('Dashboard');
  });

  it('Creates "/login" snapshot', () => {
    cy.visit('http://localhost:8000/#/login');
    cy.percySnapshot('Login');
  });

  it('Creates "/report" snapshot', () => {
    cy.visit('http://localhost:8000/#/report');
    cy.percySnapshot('Report');
  });

  it('Creates "/settings" snapshot', () => {
    cy.visit('http://localhost:8000/#/settings');
    cy.percySnapshot('Settings');
  });

  it('Creates "/user" snapshot', () => {
    cy.visit('http://localhost:8000/#/user');
    cy.percySnapshot('User');
  });
});
