export default function MainContainer({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className="flex flex-col items-center justify-center w-full py-2">
      {children}
    </div>
  );
}
