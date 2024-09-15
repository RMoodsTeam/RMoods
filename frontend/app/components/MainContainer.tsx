import { Sheet } from "@mui/joy";

/**
 * Main div container for all content.
 * Centers children vertically and horizontally.
 */
const MainContainer = ({
  children,
}: Readonly<{ children: React.ReactNode }>) => {
  return (
    <Sheet
      sx={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        width: "100vw",
        paddingX: 25,
      }}
    >
      {children}
    </Sheet>
  );
};

export default MainContainer;
