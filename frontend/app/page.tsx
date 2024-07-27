"use client";

import { useState } from "react";
import Login from "./components/Login";
/**
 * Home component, returns main content
 * @returns {TSX}
 */
export default function Home() {
  const [count, setCount] = useState(0);
  return (
    <div>
      <h1>RMoods</h1>
      <Login />
    </div>
  );
}
