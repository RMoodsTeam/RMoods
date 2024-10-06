import { Button, useColorMode } from "@chakra-ui/react";
import { getNextMode, getSystemMode, RMoodsColorMode } from "../../theme";
import { useAtom } from "jotai";
import { useEffect } from "react";
import { atomWithStorage } from "jotai/utils";

// Initialize the atom using localStorage's current value, fallback to "light"
const colorModeAtom = atomWithStorage<RMoodsColorMode>(
  "COLOR_MODE",
  (localStorage.getItem("COLOR_MODE") as RMoodsColorMode) || "light",
);

const ThemeSwitch = () => {
  // ChakraUI handles only light and dark modes, so we need to keep track of the true color mode
  const { colorMode, toggleColorMode } = useColorMode();
  const [trueColorMode, setTrueColorMode] = useAtom(colorModeAtom);

  const handleSystemSchemeChange = () => {
    const systemMode = getSystemMode();
    if (trueColorMode === "system" && colorMode !== systemMode) {
      toggleColorMode();
    }
  };

  const onThemeChange = () => {
    const nextMode = getNextMode(trueColorMode);
    const systemMode = getSystemMode();

    setTrueColorMode(nextMode);

    // Toggle ChakraUI color mode if it's different from the desired true mode
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
      <Button onClick={onThemeChange}>
        {trueColorMode === "system"
          ? `System (${getSystemMode()})`
          : trueColorMode.charAt(0).toUpperCase() + trueColorMode.slice(1)}
      </Button>
    </header>
  );
};

export default ThemeSwitch;
