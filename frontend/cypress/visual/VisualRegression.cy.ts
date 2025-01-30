// Whenever a new page is created, we can add a new test to take a snapshot of the page
describe('Visual Regression tests', () => {
  it('Creates "/" snapshot', () => {
    cy.visitPageAndScreenshotIt('');
  });

  it('Creates "/about/faq" snapshot', () => {
    cy.visitPageAndScreenshotIt('about/faq');
  });

  it('Creates "/about/thesis" snapshot', () => {
    cy.visitPageAndScreenshotIt('about/thesis');
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

  it('Creates "/user/reports" snapshot', () => {
    cy.visitPageAndScreenshotIt('user/reports');
  });

  it('Creates "/browse/reports" snapshot', () => {
    cy.visitPageAndScreenshotIt('browse/reports');
  });

  it('Creates "/browse/users" snapshot', () => {
    cy.visitPageAndScreenshotIt('browse/users');
  });

  it('Creates "/sandbox" snapshot', () => {
    cy.visitPageAndScreenshotIt('sandbox');
  });

  it('Creates "/releases" snapshot', () => {
    cy.visitPageAndScreenshotIt('releases');
  });
});
