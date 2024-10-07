import {
  Box,
  Button,
  VStack,
  Heading,
  Collapse,
  useDisclosure,
} from "@chakra-ui/react";
import { FaChevronLeft, FaChevronRight } from "react-icons/fa";
import { useState } from "react";

const Sidebar = ({ title, children, width = "300px" }: any) => {
  const { isOpen, onToggle } = useDisclosure({ defaultIsOpen: true });
  const [isCollapsed, setIsCollapsed] = useState(false);

  const handleToggle = () => {
    if (isCollapsed) {
      setIsCollapsed(false);
      setTimeout(onToggle, 100); // Delay to ensure smooth animation
    } else {
      onToggle();
      setTimeout(() => setIsCollapsed(true), 300); // Delay to match transition duration
    }
  };

  return (
    <Box
      margin={0}
      bg="gray"
      width={isCollapsed ? "50px" : width}
      boxShadow="lg"
      transition="width 0.3s ease"
      zIndex={1000}
    >
      <Button position="fixed" top="50%" onClick={handleToggle}>
        {isCollapsed ? <FaChevronLeft /> : <FaChevronRight />}
      </Button>

      <Collapse in={isOpen} animateOpacity>
        <VStack align="stretch" p={4} spacing={4}>
          <Heading size="md">{title}</Heading>
          {children}
        </VStack>
      </Collapse>
    </Box>
  );
};

export default Sidebar;
