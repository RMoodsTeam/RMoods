import React, { useState } from "react";
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
  const title = (
    <div className="flex flex-row">
      <TbPaint className="mr-2" size={30} />
    </div>
  );

  const [isOpen, setIsOpen] = useState(false);
  const onToggle = () => setIsOpen(!isOpen);
  const onOptionClick = (e: MouseEvent) => {
    setSelectedTheme(e);
    setIsOpen(false);
  };
  const iconSize = 24;

  const selectorFn = (target: string | null) => {
    return () => {
      return localStorage.getItem("theme") === target;
    };
  };

  return (
    <div>
<Dropdown isOpen={isOpen} onToggle={onToggle} title={title}>
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
    </div>
    
  );
};

export default ThemeSwitch;
