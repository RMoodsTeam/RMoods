"use client";

import { useState } from "react";
import Login from "./components/Login";

/**
 * Home function, returns main content
 * @returns {TSX}
 */
export default function Home() {
  const [count, setCount] = useState(0);
  return (
    <div>
      <Login />
    </div>
  );
}
