import React from "react";
import { Children } from "./types";

type CardProps = React.HTMLAttributes<HTMLDivElement> & Children;

/**
 * Card component with default styling
 * @param Object contains children, className and props -
 * @returns Element
 */
const Card = ({ children, className, ...props }: CardProps) => {
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
};

export default Card;
