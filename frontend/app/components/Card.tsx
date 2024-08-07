import React from "react";

export default function Card({ children, className = "", ...props }: any) {
  return (
    <div
      className={
        "bg-secondary-light dark:bg-secondary-dark shadow-md rounded-lg p-4 m-4 " +
        className
        
      }
      {...props}
    >
      {children}
    </div>
  );
}
