import React from "react";
import { Children } from "./types";

type TitleProps = React.HTMLAttributes<HTMLHeadingElement> & Children;

export default function Title({ children, ...props }: TitleProps) {
  return (
    <h1
      className={
        "text-3xl font-bold text-primary-dark:text-primary-light " + props.className
      }
      {...props}
    >
      {children}
    </h1>
  );
}
