describe("Theme Switch - user-selected `light`, system `dark`", () => {
  beforeEach(() => {
    cy.visit("localhost:8000/", {
      onBeforeLoad(win) {
        cy.stub(win, "matchMedia")
          .withArgs("(prefers-color-scheme: dark)")
          .returns({
            matches: true,
            addEventListener: () => {},
          });

        cy.stub(win.localStorage, "getItem").withArgs("theme").returns("light");
      },
    });
  });

  it("Exists", () => {
    cy.get("[data-cy=dropdown-toggle-button-theme-switch]").should("exist");
  });

  it("Should be light by default (as set by user)", () => {
    cy.get(":root").should("not.have.class", "dark");
  });

  it("Switches from user-selected light to user-selected dark", () => {
    cy.wait(500);
    cy.get("[data-cy=dropdown-toggle-button-theme-switch]").trigger("click");
    cy.get("[data-cy=option-dark]").trigger("click");
    cy.get(":root").should("have.class", "dark");
  });
});

describe("Theme Switch - user-selected `system`, system `dark`", () => {
  beforeEach(() => {
    cy.clearLocalStorage();
    cy.visit("localhost:8000/", {
      onBeforeLoad(win) {
        cy.stub(win, "matchMedia")
          .withArgs("(prefers-color-scheme: dark)")
          .returns({
            matches: true,
            addEventListener: () => {},
          });

        //cy.stub(win.localStorage, "getItem").withArgs("theme").returns(null);
      },
    });
  });

  it("Exists", () => {
    cy.get("[data-cy=dropdown-toggle-button-theme-switch]").should("exist");
  });

  it("Should be dark, as set by system", () => {
    cy.wait(500);
    cy.get("[data-cy=dropdown-toggle-button-theme-switch]").trigger("click");
    cy.get("[data-cy=option-system]").trigger("click");
    cy.get(":root").should("have.class", "dark");
  });

  it("Switches from system to user-selected light", () => {
    cy.wait(500);
    cy.get("[data-cy=dropdown-toggle-button-theme-switch]").trigger("click");
    cy.get("[data-cy=option-light]").trigger("click");
    cy.get(":root").should("not.have.class", "dark");
  });
});
