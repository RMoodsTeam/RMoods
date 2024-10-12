import {
  Box,
  Button,
  VStack,
  Heading,
  Collapse,
  useDisclosure, Card,
} from "@chakra-ui/react";
import { FaChevronLeft, FaChevronRight } from "react-icons/fa";

const Sidebar = ({ title, children, width = "300px" }: any) => {
  const { isOpen, onToggle } = useDisclosure({ defaultIsOpen: true });

  const handleToggle = () => {
    if (!isOpen) {
      setTimeout(onToggle, 100); // Delay to ensure smooth animation
    } else {
      onToggle();
    }
  };

  return (
    <Card
      margin={0}
      minWidth={isOpen ? width : "50px"}
      transition="min-width 0.3s ease"
      zIndex={1000}
      marginY="2px"
    >
      <Button position="fixed" top="50%" onClick={handleToggle}>
        {isOpen ? <FaChevronLeft /> : <FaChevronRight />}
      </Button>

      <Collapse in={isOpen} animateOpacity>
        <VStack align="stretch" p={4} spacing={4}>
          <Heading size="md">{title}</Heading>
          {children}
        </VStack>
      </Collapse>
    </Card>
  );
};

export default Sidebar;
