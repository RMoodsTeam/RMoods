import React from "react";
import { LinkProps as NextLinkProps, default as NextLink } from "next/link";
import { Children } from "./types";

type LinkProps = NextLinkProps &
  React.RefAttributes<HTMLAnchorElement> &
  Children & {
    className?: string;
  };

/**
 * Link component
 * @param props -Object containing props
 * @returns Element
 */
const Link = (props: LinkProps) => {
  return (
    <NextLink
      {...props}
      className={
        "text-blue-600 hover:text-blue-900 dark:text-blue-500 dark:hover:text-blue-400 hover:underline " +
        props.className
      }
      href={props.href}
      onClick={props.onClick}
    >
      {props.children}
    </NextLink>
  );
};

export default Link;
