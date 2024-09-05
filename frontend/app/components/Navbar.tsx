import React from "react";
import ThemeSwitch from "./ThemeSwitch";
import Link from "./Link";
import User from "./User";

/**
 * Navbar contains components to navigate the website
 */
const Navbar = () => {
  return (
    <nav>
      <Link href="/">Main</Link>
      <ThemeSwitch className="col-start-3" />
      <User>User</User>
    </nav>
  );
};

export default Navbar;
