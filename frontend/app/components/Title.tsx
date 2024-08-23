import React from "react";
import { Children } from "./types";

type TitleProps = React.HTMLAttributes<HTMLHeadingElement> & Children;

const Title = ({ children, className, ...props }: TitleProps) => {
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
};

export default Title;
