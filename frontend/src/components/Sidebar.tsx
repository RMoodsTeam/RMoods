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
      zIndex={1}
      marginY="2px"
      overflow="hidden"
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
    </Card>
  );
};

export default Sidebar;