"use client";
import React, { MouseEventHandler, useState } from "react";
import { Dropdown, DropdownOption } from "./Dropdown";
import { TbSun } from "react-icons/tb";
import { TbMoon } from "react-icons/tb";
import { TbDeviceDesktopCog } from "react-icons/tb";
import { TbPaint } from "react-icons/tb";

/**
 * setSelectedTheme function sets the theme based on the target id
 *
 * @param e - gets the event to get the target id
 */
const setSelectedTheme = (e: any) => {
  const id = e.target.id;
  if (id === "light") {
    localStorage.setItem("theme", "light");
    document.documentElement.classList.remove("dark");
  } else if (id === "dark") {
    localStorage.theme = "dark";
    document.documentElement.classList.add("dark");
  } else if (id === "system") {
    const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");
    localStorage.removeItem("theme");
    if (darkThemeMq.matches) document.documentElement.classList.add("dark");
    else document.documentElement.classList.remove("dark");
  } else {
    throw new Error(`Unrecognized theme option: ${JSON.stringify(id)}`);
  }
};

type ThemeSwitchProps = React.HTMLAttributes<HTMLDivElement>;

/**
 * ThemeSwitch component allows user to switch between light, dark and system theme
 *
 * @param className gives ability to add custom CSS classes -
 * @returns Element
 */
const ThemeSwitch = ({ className }: ThemeSwitchProps) => {
  const dropdownHeader = (
    <div className="flex flex-row">
      <TbPaint className="mr-2" size={30} />
    </div>
  );

  const [isOpen, setIsOpen] = useState(false);
  const onToggle = () => setIsOpen(!isOpen);
  const onOptionClick: MouseEventHandler = (event) => {
    setSelectedTheme(event);
    setIsOpen(false);
  };
  const iconSize = 24;

  const selectorFn = (target: string | null) => {
    return () => {
      return localStorage.getItem("theme") === target;
    };
  };

  return (
    <Dropdown
      className={className}
      isOpen={isOpen}
      onToggle={onToggle}
      header={dropdownHeader}
      id="theme-switch"
    >
      <DropdownOption
        id="light"
        onClick={onOptionClick}
        isSelected={selectorFn("light")}
      >
        <TbSun className="mr-2 my-1" size={iconSize} />
        Light
      </DropdownOption>

      <DropdownOption
        id="dark"
        onClick={onOptionClick}
        isSelected={selectorFn("dark")}
      >
        <TbMoon className="mr-2 my-1" size={iconSize} />
        Dark
      </DropdownOption>

      <DropdownOption
        id="system"
        onClick={onOptionClick}
        isSelected={selectorFn(null)}
      >
        <TbDeviceDesktopCog className="mr-2 my-1" size={iconSize} />
        System
      </DropdownOption>
    </Dropdown>
  );
};

export default ThemeSwitch;
