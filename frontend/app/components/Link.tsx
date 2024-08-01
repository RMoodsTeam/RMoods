import React from "react";
import { default as NextLink } from "next/link";

export default function Link({
  href,
  children,
  className,
  ...props
}: {
  href: string;
  children: any;
  className?: string;
  props?: any;
}) {
  return (
    <NextLink
      className={
        "text-blue-600 hover:text-blue-900 dark:text-blue-500 dark:hover:text-blue-400 hover:underline " +
        className
      }
      href={href}
      {...props}
    >
      {children}
    </NextLink>
  );
}
