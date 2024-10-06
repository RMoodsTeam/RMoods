import { Button, useColorMode } from "@chakra-ui/react";
import {
  DEFAULT_COLOR_MODE,
  getNextMode,
  getSystemMode,
  RMoodsColorMode,
} from "../../theme";
import { useState } from "react";

const ThemeSwitch = () => {
  if (!localStorage.getItem("COLOR_MODE")) {
    localStorage.setItem("COLOR_MODE", DEFAULT_COLOR_MODE);
  }
  let currentMode = localStorage.getItem("COLOR_MODE");
  const [trueColorMode, setTrueColorMode] = useState(currentMode);

  const { colorMode, toggleColorMode } = useColorMode();
  if (!colorMode) {
    return <></>;
  }

  const onSystemColorSchemeChange = () => {
    const newPreference = getSystemMode();
    if (localStorage.getItem("COLOR_MODE") === "system") {
      if (newPreference !== colorMode) {
        toggleColorMode();
      }
    }
  };

  const handleThemeChange = () => {
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", () => onSystemColorSchemeChange());
    const currentMode = (localStorage.getItem("COLOR_MODE") ||
      "light") as RMoodsColorMode;
    const nextMode = getNextMode(currentMode);
    const systemPreference = getSystemMode();
    localStorage.setItem("COLOR_MODE", nextMode);
    setTrueColorMode(nextMode);
    if (nextMode === "system") {
      if (colorMode !== systemPreference) {
        toggleColorMode();
      }
    } else if (nextMode !== colorMode) {
      toggleColorMode();
    }
  };

  return (
    <header>
      <Button onClick={() => handleThemeChange()}>{trueColorMode}</Button>
    </header>
  );
};

export default ThemeSwitch;
