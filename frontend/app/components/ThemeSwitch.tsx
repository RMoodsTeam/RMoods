"use client";
import React from "react";
import { Dropdown, DropdownOption } from "./Dropdown";
import { TbSun } from "react-icons/tb";
import { TbMoon } from "react-icons/tb";
import { TbDeviceDesktopCog } from "react-icons/tb";
import { TbPaint } from "react-icons/tb";

const setSelectedTheme = (e: any) => {
  const id = e.target.id;
  if (id === "light") {
    localStorage.setItem("theme", "light");
    document.documentElement.classList.remove("dark");
  } else if (id === "dark") {
    localStorage.theme = "dark";
    document.documentElement.classList.add("dark");
  } else {
    const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");
    localStorage.removeItem("theme");
    if (darkThemeMq.matches) document.documentElement.classList.add("dark");
    else document.documentElement.classList.remove("dark");
  }
};

const ThemeSwitch = () => {
  return (
    <Dropdown title={<TbPaint />}>
      <DropdownOption id="light" onClick={setSelectedTheme}>
        <TbSun className="mr-2" />
        Light
      </DropdownOption>
      <DropdownOption id="dark" onClick={setSelectedTheme}>
        <TbMoon className="mr-2" />
        Dark
      </DropdownOption>
      <DropdownOption id="system" onClick={setSelectedTheme}>
        <TbDeviceDesktopCog className="mr-2" />
        System
      </DropdownOption>
    </Dropdown>
  );
};

export default ThemeSwitch;
