import React from "react";
import { Children } from "./types";

type TitleProps = React.HTMLAttributes<HTMLHeadingElement> & Children;

export default function Title({ children, className, ...props }: TitleProps) {
  return (
    <h1
      className={
        "text-3xl font-bold " + className
      }
      {...props}
    >
      {children}
    </h1>
  );
}
