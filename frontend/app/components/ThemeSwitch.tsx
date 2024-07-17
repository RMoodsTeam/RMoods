"use client";
import React from "react";

/**
 * function to switch themes, uses localStorage
 * checks if there is a theme named dark or if there is no theme variable
 * @returns void
 */
function switchThemes() {
  if (localStorage.theme === "dark") {
    localStorage.removeItem("theme");
    document.documentElement.classList.remove("dark");
  } else {
    localStorage.theme = "dark";
    document.documentElement.classList.add("dark");
  }
}

/**
 * Renders button that switches themes
 */
const ThemeSwitch = () => {
  return (
    <button
      className="bg-primary-light dark:bg-primary-dark"
      onClick={() => {
        switchThemes();
      }}
    >
      Change theme here!
    </button>
  );
};

export default ThemeSwitch;
