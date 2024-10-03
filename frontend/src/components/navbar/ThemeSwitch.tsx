import { Button, useColorMode } from "@chakra-ui/react";

const ThemeSwitch = () => {
  const { colorMode, toggleColorMode } = useColorMode();
  return (
    <Button onClick={toggleColorMode}>
      Toggle to {colorMode === "light" ? "Dark" : "Light"}
    </Button>
  );
};

export default ThemeSwitch;
