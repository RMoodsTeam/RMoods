"use client";
import React, { useState } from "react";

/**
 * function to switch themes, uses localStorage
 * for now switches between dark and light themes
 * @returns void
 */
export function switchThemes() {
  if (localStorage.theme === "dark") {
    localStorage.theme = "light";
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
      id="theme-switch"
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
