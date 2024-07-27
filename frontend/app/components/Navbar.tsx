import Link from "next/link";
import React from "react";

/**
 * Navbar component containing Links to other pages
 */
const Navbar = () => {
  return (
    <nav className="sticky top-0 z-50">
      <Link href="/dashboard">Dashboard</Link>
      <Link href="/">Main</Link>
    </nav>
  );
};

export default Navbar;
