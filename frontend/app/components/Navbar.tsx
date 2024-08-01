"use client";
import Link from "next/link";
import React from "react";
import Cookies from "js-cookie";

/**
 * Navbar component containing Links to other pages
 */
const Navbar = () => {
  return (
    <nav className="sticky top-0 z-50">
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
    </nav>
  );
};

export default Navbar;
