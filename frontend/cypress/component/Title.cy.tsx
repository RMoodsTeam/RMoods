import Home from "../../app/page";

describe("Title", () => {
  it("h1 should contain title", () => {
    cy.mount(<Home />);
    cy.get("h1").should("contain.text", "RMoods");
  });
});
