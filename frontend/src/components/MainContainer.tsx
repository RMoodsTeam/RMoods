import { Box } from "@chakra-ui/react";

const flexMain = {
  display: "flex",
  flexDirection: "column",
  justifyContent: "center",
  alignItems: "center",
  width: "100vw",
  paddingX: "20%",
};

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
        ...flexMain,
      }}
    >
      {children}
    </Box>
  );
};

export default MainContainer;
