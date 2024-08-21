import React from "react";
import ThemeSwitch from "./ThemeSwitch";
import Link from "./Link";
import Logout from "./Logout";

/**
 * Navbar component containing Links to other pages
 */
const Navbar = () => {
  return (
    <nav className="bg-primary-light dark:bg-primary-dark sticky top-0 z-50 w-full p-4 shadow-md grid grid-cols-[100px_100px_1fr_100px_100px] place-items-center">
      <Link href="/">Main</Link>
      {/* TODO!! move button to another file so the whole navbar is not a client component */}
      <Logout />
      <ThemeSwitch className="col-start-4" />
      <Link href="/dashboard">Dashboard</Link>
    </nav>
  );
};

export default Navbar;
