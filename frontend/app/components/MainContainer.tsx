import { Box } from "@mui/joy";

/**
 * Main div container for all content.
 * Centers children vertically and horizontally.
 */
const MainContainer = ({
  children,
}: Readonly<{ children: React.ReactNode }>) => {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        width: "100%",
      }}
    >
      {children}
    </Box>
  );
};

export default MainContainer;
