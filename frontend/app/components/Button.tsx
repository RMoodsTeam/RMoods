import React from "react";

export default function Button({ children, ...props }: any) {
  return (
    <button
      className="bg-accent-purple hover:bg-accent-green dark:bg-accent-green dark:hover:bg-accent-purple text-white font-bold py-2 px-4 rounded"
      {...props}
    >
      {children}
    </button>
  );
}
