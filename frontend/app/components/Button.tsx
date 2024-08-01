import React from "react";

export default function Button({ children, ...props }: any) {
  const light = "bg-accent-purple hover:bg-accent-green ";
  const dark = "dark:bg-accent-green dark:hover:bg-accent-purple ";
  return (
    <button
      className={light + dark + "text-white font-bold py-2 px-4 rounded"}
      {...props}
    >
      {children}
    </button>
  );
}
