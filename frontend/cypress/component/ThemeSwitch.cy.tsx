import React from "react";
import ThemeSwitch from "../../app/components/ThemeSwitch";

describe("Theme switch button", () => {
  it("renders", () => {
    cy.mount(<ThemeSwitch />);
    cy.get("button")
      .click()
      .should(() => {
        expect(localStorage.getItem("theme")).to.eq("dark");
      });
  });
});
