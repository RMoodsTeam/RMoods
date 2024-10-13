import {
  VStack,
  Heading,
  Card,
} from "@chakra-ui/react";

const Sidebar = () => {

  return (
      <Card
          margin={0}
          minWidth="300px"
          zIndex={1000}
          marginY="2px"
      >
          <VStack align="stretch" p={4} spacing={4}>
            <Heading size="md">Sidebar</Heading>
          </VStack>
      </Card>
  );
};

export default Sidebar;