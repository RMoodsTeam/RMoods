import { Flex } from "@chakra-ui/react";
import Sidebar from "./Sidebar";

const flexMain = {
  flex: "auto",
  flexDirection: "column",
  justifyContent: "center",
  alignItems: "center",
  padding: "60px 1em 0",
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
    sx={{flex: "auto"}}>
      <Sidebar />
      <Flex
        sx={{
          ...flexMain,
        }}
      >
        {children}
      </Flex>
    </Flex>
  );
};

export default MainContainer;
