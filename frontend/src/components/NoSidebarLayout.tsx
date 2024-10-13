import {Flex} from "@chakra-ui/react";

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

const NoSidebarLayout = ({
                           children,
                         }: Readonly<{ children: React.ReactNode }>) => {
  return (
    <Flex
      sx={{flex: "auto"}}>
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

export default NoSidebarLayout;
