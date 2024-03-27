"use client";

import { useState } from "react";

export default function Home() {
  const [count, setCount] = useState(0);
  return (
    <>
      <h1>XMoods</h1>
      <button onClick={() => setCount(count + 1)}>Count is {count}</button>
    </>
  );
}
