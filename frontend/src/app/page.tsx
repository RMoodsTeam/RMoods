"use client";

import { useState } from "react";

/**
 * Home function, returns main content
 * @returns {TSX}
 */
export default function Home() {
  const [count, setCount] = useState(0);
  return (
    <>
      <h1>RMoods</h1>
      <button onClick={() => setCount(count + 1)}>Count is {count}</button>
    </>
  );
}
