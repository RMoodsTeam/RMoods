import React, { useState } from "react";
import Card from "./Card";

export const DropdownOption = ({ id, children, onClick }: any) => {
  return (
    <div className="flex items-center" onClick={onClick} id={id}>
      {children}
    </div>
  );
};

export const Dropdown = ({
  children,
  className = "",
  title,
  ...props
}: any) => {
  const [isOpen, setIsOpen] = useState(false);

  const toggleDropdown = () => {
    setIsOpen(!isOpen);
  };

  return (
    <div>
      <button onClick={toggleDropdown} className="relative">
        {title}
      </button>
      {isOpen ? <Card className="absolute top-10">{...children}</Card> : <></>}
    </div>
  );
};
