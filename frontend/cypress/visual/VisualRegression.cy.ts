// Whenever a new page is created, we can add a new test to take a snapshot of the page
describe('Visual Regression tests', () => {
  it('Creates "/" snapshot', () => {
    cy.visitPageAndScreenshotIt('');
  });

  it('Creates "/about/faq" snapshot', () => {
    cy.visitPageAndScreenshotIt('about/faq');
  });

  it('Creates "/dashboard" snapshot', () => {
    cy.visitPageAndScreenshotIt('dashboard');
  });

  it('Creates "/login" snapshot', () => {
    cy.visitPageAndScreenshotIt('login');
  });

  it('Creates "/report" snapshot', () => {
    cy.visitPageAndScreenshotIt('report');
  });

  it('Creates "/settings" snapshot', () => {
    cy.visitPageAndScreenshotIt('settings');
  });

  it('Creates "/user" snapshot', () => {
    cy.visitPageAndScreenshotIt('user');
  });
});
