import {
  VStack,
  Heading,
  Card, useDisclosure, Box, Collapse, Icon, Flex,
} from "@chakra-ui/react";
import {FaChevronDown, FaChevronRight} from "react-icons/fa";
import React from "react";


const SidebarSection = ({sectionTitle, children}: {sectionTitle: string, children: React.ReactNode}) => {
  const {isOpen, onToggle} = useDisclosure()

  return (
    <>
      <Flex onClick={onToggle}
      >
        <Icon as={isOpen ? FaChevronDown : FaChevronRight}/>
        {sectionTitle}
      </Flex>
      <Collapse in={isOpen} animateOpacity>
        <Box
          marginLeft="12px"
        >
          {children}
        </Box>
      </Collapse>
    </>
  )
}

const Sidebar = () => {
  return (
    <Card
      margin={0}
      minWidth="200px"
      maxWidth="200px"
      height="100vh"
      zIndex={1}
      overflow="hidden"
      borderRadius={0}
      position="sticky" // Make the sidebar stick to the top
      top={0}
    >
      <Box
        height="100vh"
        overflowY="auto"
        padding="16px"
      >
        <Heading size="md">Sidebar</Heading>
        <SidebarSection sectionTitle="Lorem">
          <Box>Lorem ipsum dolor sit amet, consectetur adipisicing elit</Box>
          <SidebarSection sectionTitle="Ipsum">
            <Box>Assumenda, quia temporibus eveniet a libero incidunt suscipit</Box>
            <SidebarSection sectionTitle="Ipsum">
              <Box>Assumenda, quia temporibus eveniet a libero incidunt suscipit</Box>
            </SidebarSection>
          </SidebarSection>
          <Box>Quidem, ipsam illum quis sed voluptatum quae eum fugit earum</Box>
          <Box>Quidem, ipsam illum quis sed voluptatum quae eum fugit earum</Box>
        </SidebarSection>
      </Box>
    </Card>
  );
};

export default Sidebar;