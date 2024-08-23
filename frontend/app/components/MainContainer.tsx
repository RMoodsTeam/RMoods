const MainContainer = ({
  children,
}: Readonly<{ children: React.ReactNode }>) => {
  return (
    <div className="flex flex-col px-64 items-center justify-center w-full py-2">
      {children}
    </div>
  );
};

export default MainContainer;
