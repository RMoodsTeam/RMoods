import { Flex } from "@chakra-ui/react";

const flexMain = {
  flexDirection: "column",
  justifyContent: "center",
  alignItems: "center",
  paddingX: "60px",
};

/**
 * Main div container for all content.
 * Centers children vertically and horizontally.
 */

const MainContainer = ({
  children,
}: Readonly<{ children: React.ReactNode }>) => {
  return (
    <Flex
      sx={{
        ...flexMain,
      }}
    >
      {children}
    </Flex>
  );
};

export default MainContainer;
