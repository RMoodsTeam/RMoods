import React from "react";
import Card from "./Card";

export const DropdownOption = ({ id, children, onClick, isSelected }: any) => {
  const styling = isSelected()
    ? "text-accent-purple dark:text-accent-green font-bold" // when selected
    : "text-primary-dark dark:text-primary-light"; // when not selected
  return (
    <div
      className={
        "flex items-center w-full p-2 relative hover:bg-gray-300 dark:hover:bg-slate-600 cursor-pointer rounded-lg " +
        styling
      }
      onClick={onClick}
      id={id}
    >
      {children}
    </div>
  );
};

export const Dropdown = ({
  children,
  isOpen,
  onToggle,
  className = "",
  title,
  ...props
}: any) => {
  return (
    <div>
      <button onClick={onToggle} className="relative">
        <div className="flex flex-row">{title}</div>
      </button>
      {isOpen ? (
        <Card
          className="absolute -translate-x-9 translate-y-2 !m-0 !p-0 "
          {...props}
        >
          {...children}
        </Card>
      ) : (
        <></>
      )}
    </div>
  );
};
