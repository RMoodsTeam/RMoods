import { Button, Icon, useColorMode, HStack } from "@chakra-ui/react";
import { FaSun, FaMoon, FaDesktop } from "react-icons/fa";
import { getNextMode, getSystemMode } from "../../theme";
import { useAtom } from "jotai";
import { useEffect } from "react";
import { colorModeAtom } from "../../atoms";

const ThemeSwitch = () => {
  const { colorMode, toggleColorMode } = useColorMode();
  const [trueColorMode, setTrueColorMode] = useAtom(colorModeAtom);

  // Determine the current icon based on the mode
  const getIcon = () => {
    switch (trueColorMode) {
      case "light":
        return FaSun;
      case "dark":
        return FaMoon;
      case "system":
        return FaDesktop;
      default:
        return FaSun;
    }
  };

  // Handle system color scheme preference changes
  const handleSystemSchemeChange = () => {
    const systemMode = getSystemMode();
    if (trueColorMode === "system" && colorMode !== systemMode) {
      toggleColorMode();
    }
  };

  // Handle theme change logic
  const onThemeChange = () => {
    const nextMode = getNextMode(trueColorMode);
    const systemMode = getSystemMode();

    setTrueColorMode(nextMode);

    if (
      (nextMode === "system" && colorMode !== systemMode) ||
      (nextMode !== "system" && nextMode !== colorMode)
    ) {
      toggleColorMode();
    }
  };

  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", handleSystemSchemeChange);
    return () =>
      mediaQuery.removeEventListener("change", handleSystemSchemeChange);
  }, [colorMode, trueColorMode]);

  return (
    <header>
      <Button onClick={onThemeChange} aria-label="Toggle Theme" px={4} py={2}>
        <HStack spacing={2}>
          <Icon as={getIcon()} boxSize={5} />
        </HStack>
      </Button>
    </header>
  );
};

export default ThemeSwitch;
