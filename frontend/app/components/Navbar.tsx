"use client";
import React from "react";
import Cookies from "js-cookie";
import ThemeSwitch from "./ThemeSwitch";
import Link from "./Link";

/**
 * Navbar component containing Links to other pages
 */
const Navbar = () => {
  return (
    <nav className="bg-primary-light dark:bg-primary-dark sticky top-0 z-50 w-full p-4 shadow-md grid grid-cols-[100px_100px_1fr_100px_100px] place-items-center">
      <Link href="/">Main</Link>
      {/* TODO!! move button to another file so the whole navbar is not a client component */}
      <Link href="/login" onClick={() => Cookies.remove("RMOODS_JWT")}>
        Log out
      </Link>
      {/* TODO!! fix the div and make ThemeSwitch take arguments*/}
      <div className="col-start-4">
        <ThemeSwitch />
      </div>
      <Link href="/dashboard">Dashboard</Link>
    </nav>
  );
};

export default Navbar;
