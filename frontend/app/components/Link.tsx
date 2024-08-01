import React from "react";
import { default as NextLink } from "next/link";

export default function Link({
  href,
  children,
  ...props
}: {
  href: string;
  children: any;
  props?: any;
}) {
  return (
    <NextLink
      className="text-blue-600 hover:text-blue-900 dark:text-blue-500 dark:hover:text-blue-400 hover:underline"
      href={href}
      {...props}
    >
      {children}
    </NextLink>
  );
}
