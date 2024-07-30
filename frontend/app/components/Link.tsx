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
      className="text-blue-600 hover:text-blue-900 hover:underline"
      href={href}
      {...props}
    >
      {children}
    </NextLink>
  );
}
