import {
  Button,
  Icon,
  useColorMode,
  HStack,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
} from "@chakra-ui/react";
import { FaSun, FaMoon, FaDesktop } from "react-icons/fa";
import { getSystemMode, RMoodsColorMode } from "../../theme";
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
  const onThemeChange = (newMode: RMoodsColorMode) => {
    const systemMode = getSystemMode();

    setTrueColorMode(newMode);

    if (newMode === "system") {
      if (colorMode !== systemMode) {
        toggleColorMode();
      }
    } else {
      if (colorMode !== newMode) {
        toggleColorMode();
      }
    }

    // if (
    //   (newMode === "system" && newMode !== (systemMode as RMoodsColorMode)) ||
    //   (newMode !== "system" && newMode !== colorMode)
    // ) {
    //   toggleColorMode();
    // }
  };

  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", handleSystemSchemeChange);
    return () =>
      mediaQuery.removeEventListener("change", handleSystemSchemeChange);
  }, [colorMode, trueColorMode]);

  return (
    <header>
      <Menu>
        <MenuButton as={Button} aria-label="Theme menu">
          <HStack spacing={2}>
            <Icon as={getIcon()} boxSize={5} />
          </HStack>
        </MenuButton>
        <MenuList>
          <HStack>
            <MenuItem
              onClick={() => onThemeChange("light")}
              icon={<FaSun size={16} />}
            >
              Light
            </MenuItem>
          </HStack>
          <HStack>
            <MenuItem
              onClick={() => onThemeChange("dark")}
              icon={<FaMoon size={16} />}
            >
              Dark
            </MenuItem>
          </HStack>
          <HStack minWidth={24}>
            <MenuItem
              onClick={() => onThemeChange("system")}
              icon={<FaDesktop size={16} />}
            >
              System
            </MenuItem>
          </HStack>
        </MenuList>
      </Menu>
    </header>
  );
};

export default ThemeSwitch;
