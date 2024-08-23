import React, { ReactNode } from "react";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  children: ReactNode;
};

const Button = ({ children, ...props }: ButtonProps) => {
  return (
    <button
      className="bg-accent-purple hover:bg-accent-green dark:bg-accent-green dark:hover:bg-accent-purple text-white font-bold py-2 px-4 rounded"
      {...props}
    >
      {children}
    </button>
  );
};

export default Button;
