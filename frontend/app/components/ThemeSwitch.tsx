"use client";
import React from "react";

function switchThemes() {
  // TODO: On page load or when changing themes, best to add inline in `head` to avoid FOUC
  if (
    localStorage.theme === "dark" ||
    (!("theme" in localStorage) &&
      window.matchMedia("(prefers-color-scheme: dark)").matches)
  ) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
  if (localStorage.theme === "dark") {
    localStorage.theme = "light";
  } else {
    localStorage.theme = "dark";
  }
  console.log(localStorage.theme);
}

const ThemeSwitch = () => {
  return (
    <>
      <button
        className="bg-primary-light dark:bg-primary-dark"
        onClick={() => {
          switchThemes();
        }}
      >
        Change theme here!
      </button>
    </>
  );
};

export default ThemeSwitch;
