"use client";
import React from "react";
import Link from "./Link";
import Cookies from "js-cookie";

const Logout = () => {
  return (
    <Link href="/login" onClick={() => Cookies.remove("RMOODS_JWT")}>
      Log out
    </Link>
  );
};

export default Logout;
