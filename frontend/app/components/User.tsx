"use client";
import React, { MouseEventHandler, useState } from "react";
import Link from "./Link";
import { Dropdown, DropdownOption } from "./Dropdown";
import Logout from "./Logout";

type UserProps = React.HTMLAttributes<HTMLDivElement>;

const User = ({ className }: UserProps) => {
  const userHeader = <div>User</div>;

  const [isOpen, setIsOpen] = useState(false);
  const onToggle = () => setIsOpen(!isOpen);
  const onOptionClick: MouseEventHandler = (event) => {
    setIsOpen(false);
  };
  return (
    <Dropdown
      className={className}
      isOpen={isOpen}
      onToggle={onToggle}
      header={userHeader}
      id="user-dropdown"
    >
      {/*TODO!!! Maybe change how dropdown behaves?*/}
      <DropdownOption onClick={onOptionClick} isSelected={() => false}>
        <Link href="/dashboard">Dashboard</Link>
      </DropdownOption>
      <DropdownOption onClick={onOptionClick} isSelected={() => false}>
        <Logout />
      </DropdownOption>
    </Dropdown>
  );
};

export default User;
