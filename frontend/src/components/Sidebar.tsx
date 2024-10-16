import {
  VStack,
  Heading,
  Card, useDisclosure, Box, Collapse, Icon, Flex,
} from "@chakra-ui/react";
import {FaChevronDown, FaChevronRight } from "react-icons/fa";


const SidebarSection = ({sectionTitle, children}) => {
  const {isOpen, onToggle} = useDisclosure()

  return (
    <>
      <Flex onClick={onToggle}
      >
        <Icon as={isOpen ? FaChevronDown  : FaChevronRight}/>
        {sectionTitle}
      </Flex >
      <Collapse in={isOpen} animateOpacity>
        <Box
          marginLeft="24px"
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
      width="300px"
      height="80vh" // Make the sidebar full height
      zIndex={1}
      marginY="2px"
      overflow="hidden"
      position="sticky" // Make the sidebar stick to the top
      top={0}
    >
      <Box
        height="100%"
        overflowY="auto" // Add vertical scroll to the content
        padding="16px"
      >
        <Heading size="md">Sidebar</Heading>
        <SidebarSection sectionTitle="Lorem">
          <Box>Lorem ipsum dolor sit amet, consectetur adipisicing elit</Box>
          <SidebarSection sectionTitle="Ipsum">
            <Box>Assumenda, quia temporibus eveniet a libero incidunt suscipit</Box>
            <SidebarSection sectionTitle="Ipsum">
              <Box>Assumenda, quia temporibus eveniet a libero incidunt suscipit</Box>
              <SidebarSection sectionTitle="Ipsum">
                <Box>Assumenda, quia temporibus eveniet a libero incidunt suscipit</Box>
                <SidebarSection sectionTitle="Ipsum">
                  <Box>Assumenda, quia temporibus eveniet a libero incidunt suscipit</Box>
                </SidebarSection>
              </SidebarSection>
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