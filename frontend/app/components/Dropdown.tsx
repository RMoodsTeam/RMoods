import React, { useState } from "react";
import Card from "./Card";
import { TbTriangle } from "react-icons/tb";

export const DropdownOption = ({ id, children, onClick }: any) => {
  return (
    <div className="flex items-center" onClick={onClick} id={id}>
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
        <Card className="absolute -translate-x-14 -translate-y-15" {...props}>
          {...children}
        </Card>
      ) : (
        <></>
      )}
    </div>
  );
};
