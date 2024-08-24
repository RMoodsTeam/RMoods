import React from "react";
import { Children } from "./types";

type TitleProps = React.HTMLAttributes<HTMLHeadingElement> & Children;

/**
 * Title component allows user to set the title of the page
 * @param Object  contains the children, ckassName and props
 * @returns Element
 */
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
