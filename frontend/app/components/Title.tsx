import React from "react";

export default function Title({ children, className = "", ...props }: any) {
  return (
    <h1
      className={
        "text-3xl font-bold text-primary-dark:text-primary-light " + className
      }
      {...props}
    >
      {children}
    </h1>
  );
}
