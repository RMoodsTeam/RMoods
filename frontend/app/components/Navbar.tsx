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
    <nav className="bg-primary-light dark:bg-primary-dark sticky top-0 z-50 w-full flex p-4 shadow-md place-content-around ">
      <Link href="/dashboard">Dashboard</Link>
      <Link href="/">Main</Link>
      {/* TODO!! move button to another file so the whole navbar is not a client component */}
      <Link href="/login" onClick={() => Cookies.remove("RMOODS_JWT")}>
        Log out
      </Link>
      <Link href="/about">About</Link>
      <ThemeSwitch />
    </nav>
  );
};

export default Navbar;
