"use client";
import Link from "next/link";
import React from "react";
import Cookies from "js-cookie";
import ThemeSwitch from "./ThemeSwitch";

/**
 * Navbar component containing Links to other pages
 */
const Navbar = () => {
  return (
    <nav className="sticky top-0 z-50 w-full flex justify-center p-4 shadow-md">
      <Link href="/dashboard">Dashboard</Link>
      <Link href="/">Main</Link>
      {/* TODO!! move button to another file so the whole navbar is not a client component */}
      <Link href="/login">
        <button
          onClick={() => {
            Cookies.remove("RMOODS_JWT");
          }}
        >
          Log out
        </button>
      </Link>
      <ThemeSwitch />
    </nav>
  );
};

export default Navbar;
