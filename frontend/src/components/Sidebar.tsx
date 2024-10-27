import {
  Image,
  Heading,
  Card, useDisclosure, Box, Collapse, Icon, Flex, Divider, Link,
} from "@chakra-ui/react";
import {FaChevronDown, FaChevronRight} from "react-icons/fa";
import React from "react";

const sidebarStyles = {
  margin: 0,
  minWidth: "200px",
  maxWidth: "200px",
  height: "100vh",
  zIndex: 1,
  overflow: "hidden",
  borderRadius: 0,
  position: "sticky",
  top: 0,
  paddingTop: 0,
}

const contentStyles = {
  height: "100vh",
  overflowY: "auto",
  padding: "16px"
}

const logoStyles = {
  maxHeight: "10vh",
  padding: "16px",
  marginTop: 0
}

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
      sx={sidebarStyles}
    >
      <Box
        sx={logoStyles}
      >
        <Link href="/">
        <Image
          borderRadius='full'
          boxSize='3.5rem'
          src='https://bit.ly/naruto-sage'
          alt='Naruto Uzumaki'
          display='inline-block'
        />
          {/*There will be our name further down the road, it's a placeholder for now*/}
          RMoods
        </Link>
      </Box>
      <Divider />
      <Box
        sx={contentStyles}
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