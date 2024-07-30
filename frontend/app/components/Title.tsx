export default function Title({ children, className = "", ...props }: any) {
  const defaultClasses =
    "text-3xl font-bold text-primary-dark:text-primary-light";
  const combinedClasses = `${defaultClasses} ${className}`.trim();

  return (
    <h1 className={combinedClasses} {...props}>
      {children}
    </h1>
  );
}
