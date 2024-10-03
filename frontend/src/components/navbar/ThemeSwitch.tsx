import { Button, useColorMode } from "@chakra-ui/react";

const ThemeSwitch = () => {
  const { colorMode, toggleColorMode } = useColorMode();
  return (
    <header>
      <Button onClick={toggleColorMode}>
        Toggle to {colorMode === "light" ? "Dark" : "Light"}
      </Button>
    </header>
  );
};

export default ThemeSwitch;
