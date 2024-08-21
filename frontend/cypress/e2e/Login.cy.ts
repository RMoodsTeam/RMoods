describe("Login", () => {
  beforeEach(() => {
    cy.visit("http://localhost:8000/login");
  });

  it("Displays a title", () => {
    cy.get("#login-title").should("have.text", "Sign in to RMoods");
  });

  it("Displays Google sign in button", () => {
    cy.get("#google-sign-in-button").should("be.visible");
  });

  it("Google button has the correct text", () => {
    cy.get("#google-sign-in-button-text").should(
      "have.text",
      "Sign in with Google",
    );
  });
});
