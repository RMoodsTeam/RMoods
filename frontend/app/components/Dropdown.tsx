import React, { ReactNode } from "react";
import Card from "./Card";
import { Children } from "./types";

type DropdownOptionProps = React.HTMLAttributes<HTMLButtonElement> & Children & {
  isSelected: () => boolean,
}

export const DropdownOption = ({ children, isSelected, onClick, ...props }: DropdownOptionProps) => {
  const styling = isSelected()
    ? "text-accent-purple dark:text-accent-green font-bold"// when selected
    : "text-primary-dark dark:text-primary-light"; // when not selected
  return (
    <button
      className={
        "flex items-center w-full p-2 relative hover:bg-gray-300 dark:hover:bg-slate-600 cursor-pointer rounded-lg " +
        styling
      }
      data-cy={`option-${props.id}`}
      onClick={onClick}
      {...props}
    >
      {children}
    </button>
  );
};

type DropdownProps = React.HTMLAttributes<HTMLDivElement> & Children & {
  isOpen: boolean,
  onToggle: () => void,
  header: ReactNode
}

export const Dropdown = ({
  isOpen,
  className,
  onToggle,
  header,
  children,
  ...props
}: DropdownProps) => {
  return (
    <div className={className}>
      <button onClick={onToggle} className="relative" data-cy={`dropdown-toggle-button-${props.id}`}>
        <div className="flex flex-row" data-cy={`dropdown-title-${props.id}`}>{header}</div>
      </button>
      {isOpen ? (
        <Card
          className="absolute -translate-x-9 translate-y-2 !m-0 !p-0 "
          data-cy={`dropdown-card-${props.id}`}
          id={props.id}
          {...props}
        >
          {children}
        </Card>
      ) : (
        <></>
      )}
    </div>
  );
};
