import { Dropdown, DropdownOption } from "../../app/components/Dropdown";

const createDropdown = (isOpen: boolean, onToggle?: any) => {
    return (
        <Dropdown isOpen={isOpen} onToggle={onToggle} title="Title">
            <DropdownOption isSelected={() => false}>
                First
            </DropdownOption>
            <DropdownOption isSelected={() => false}>
                Second
            </DropdownOption>
        </Dropdown>
    );
}

describe("Dropdown", () => {
    it("Hidden dropdown should not exist", () => {
        const isOpen = false;
        const dropdown = createDropdown(isOpen);

      cy.mount(dropdown);
      cy.get("[data-cy=dropdown-toggle-button]").click();
      cy.get("[data-cy=dropdown-card]").should("not.exist");
    });
    it("Clicked dropdown should be visible", () => {
        const isOpen = true;
        const dropdown = createDropdown(isOpen);

      cy.mount(dropdown);
      cy.get("[data-cy=dropdown-toggle-button]").click();
      cy.get("[data-cy=dropdown-card]").should("be.visible");
    });
    it("onToggle gets called", () => {
        const isOpen = true;
        const onToggleSpy = cy.spy().as('onToggleSpy');
        const dropdown = createDropdown(isOpen, onToggleSpy);

      cy.mount(dropdown);
      cy.get("[data-cy=dropdown-toggle-button]").click();
      cy.get("@onToggleSpy").should("have.been.called");
    });
  });