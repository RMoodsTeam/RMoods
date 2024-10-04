import React from "react";
import {
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionPanel,
  AccordionIcon,
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  VisuallyHidden,
  Alert,
  AlertIcon,
  CircularProgress,
  Progress,
  Skeleton,
  Spinner,
  useToast,
  Box,
  Button,
  AlertDialog,
  AlertDialogBody,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogContent,
  AlertDialogOverlay,
  Drawer,
  DrawerBody,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  DrawerContent,
  DrawerCloseButton,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  MenuDivider,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  Popover,
  PopoverTrigger,
  PopoverContent,
  PopoverHeader,
  PopoverBody,
  PopoverFooter,
  Tooltip,
  Avatar,
  Icon,
  Image,
  Badge,
  Card,
  Code,
  Divider,
  Kbd,
  List,
  ListItem,
  Stat,
  StatLabel,
  StatNumber,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  Tag,
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  Link,
  LinkOverlay,
  SkipNavLink,
  Step,
  StepDescription,
  StepIcon,
  StepIndicator,
  StepNumber,
  StepSeparator,
  StepStatus,
  StepTitle,
  useSteps,
  Stepper,
  Checkbox,
  Editable,
  EditableInput,
  EditablePreview,
  FormControl,
  FormLabel,
  Input,
  IconButton,
  NumberInput,
  NumberInputField,
  PinInput,
  Radio,
  RangeSlider,
  Select,
  Slider,
  Switch,
  Textarea,
  CloseButton,
  Portal,
  Show,
  Hide,
  Heading,
  Highlight,
  Text,
} from "@chakra-ui/react";
import { FaBeer } from "react-icons/fa";

const Demo: React.FC = () => {
  const toast = useToast();
  const [isOpen, setIsOpen] = React.useState(false);
  const onClose = () => setIsOpen(false);
  const [drawerOpen, setDrawerOpen] = React.useState(false);

  return (
    <Box p={8}>
      <Heading mb={6}>Chakra UI Component Showcase</Heading>

      {/* Feedback Components */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Feedback Components
        </Heading>
        <Alert status="info">
          <AlertIcon />
          This is an alert message!
        </Alert>
        <CircularProgress value={40} color="blue.400" mt={4} />
        <Progress hasStripe value={64} mt={4} />
        <Skeleton height="20px" mt={4} />
        <Spinner mt={4} />
        <Button
          onClick={() => toast({ title: "Toast message", status: "success" })}
          mt={4}
        >
          Show Toast
        </Button>
      </Box>

      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Accordion
        </Heading>
        <Accordion allowToggle>
          <AccordionItem>
            <h2>
              <AccordionButton>
                <Box flex="1" textAlign="left">
                  Accordion Header
                </Box>
                <AccordionIcon />
              </AccordionButton>
            </h2>
            <AccordionPanel pb={4}>Accordion panel content</AccordionPanel>
          </AccordionItem>
        </Accordion>
      </Box>

      {/* Tabs */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Tabs
        </Heading>
        <Tabs>
          <TabList>
            <Tab>Tab 1</Tab>
            <Tab>Tab 2</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              <p>Tab panel content 1</p>
            </TabPanel>
            <TabPanel>
              <p>Tab panel content 2</p>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Box>

      {/* Visually Hidden */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Visually Hidden
        </Heading>
        <Button>
          <VisuallyHidden>This is a visually hidden element</VisuallyHidden>
          Click Me (Hidden text)
        </Button>
      </Box>

      {/* Close Button */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Close Button
        </Heading>
        <CloseButton size="md" />
      </Box>

      {/* Show / Hide */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Show / Hide
        </Heading>
        <Show breakpoint="(min-width: 600px)">
          <Text>This text is visible on wider screens</Text>
        </Show>
        <Hide breakpoint="(min-width: 600px)">
          <Text>This text is hidden on wider screens</Text>
        </Hide>
      </Box>

      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Stepper
        </Heading>
        <Stepper size="lg" index={1}>
          <Step>
            <StepIndicator>
              <StepStatus />
            </StepIndicator>
            <StepTitle>Step 1</StepTitle>
            <StepSeparator />
          </Step>
          <Step>
            <StepIndicator>
              <StepStatus />
            </StepIndicator>
            <StepTitle>Step 2</StepTitle>
            <StepSeparator />
          </Step>
        </Stepper>
      </Box>

      {/* Popover with Footer */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Popover
        </Heading>
        <Popover>
          <PopoverTrigger>
            <Button>Open Popover</Button>
          </PopoverTrigger>
          <PopoverContent>
            <PopoverHeader>Popover Header</PopoverHeader>
            <PopoverBody>This is the body of the Popover</PopoverBody>
            <PopoverFooter>Popover Footer</PopoverFooter>
          </PopoverContent>
        </Popover>
      </Box>

      {/* Link Overlay */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Link Overlay
        </Heading>
        <Card>
          <LinkOverlay href="https://chakra-ui.com" isExternal>
            <Heading as="h3" size="lg">
              Chakra UI
            </Heading>
          </LinkOverlay>
          <Text>This card contains a link overlay.</Text>
        </Card>
      </Box>

      {/* Remaining Components */}
      {/* Feedback, Media, Navigation, Form Components, etc. */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Toast
        </Heading>
        <Button
          onClick={() =>
            toast({ title: "Success!", status: "success", duration: 2000 })
          }
        >
          Show Toast
        </Button>
      </Box>

      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Drawer
        </Heading>
        <Button onClick={() => setDrawerOpen(true)}>Open Drawer</Button>
        <Drawer
          isOpen={drawerOpen}
          placement="right"
          onClose={() => setDrawerOpen(false)}
        >
          <DrawerOverlay />
          <DrawerContent>
            <DrawerCloseButton />
            <DrawerHeader>Drawer Title</DrawerHeader>
            <DrawerBody>Drawer body content</DrawerBody>
            <DrawerFooter>
              <Button onClick={() => setDrawerOpen(false)}>Close Drawer</Button>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>
      </Box>

      {/* Overlay Components */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Overlay Components
        </Heading>
        <Button onClick={() => setIsOpen(true)}>Open Alert Dialog</Button>
        {/* <AlertDialog */}
        {/*   isOpen={isOpen} */}
        {/*   onClose={onClose} */}
        {/*   leastDestructiveRef={undefined} */}
        {/* > */}
        {/*   <AlertDialogOverlay> */}
        {/*     <AlertDialogContent> */}
        {/*       <AlertDialogHeader>Alert Dialog</AlertDialogHeader> */}
        {/*       <AlertDialogBody> */}
        {/*         This is an alert dialog message. */}
        {/*       </AlertDialogBody> */}
        {/*       <AlertDialogFooter> */}
        {/*         <Button onClick={onClose}>Close</Button> */}
        {/*       </AlertDialogFooter> */}
        {/*     </AlertDialogContent> */}
        {/*   </AlertDialogOverlay> */}
        {/* </AlertDialog> */}

        <Drawer isOpen={isOpen} onClose={onClose}>
          <DrawerOverlay />
          <DrawerContent>
            <DrawerCloseButton />
            <DrawerHeader>Drawer Header</DrawerHeader>
            <DrawerBody>Drawer body content...</DrawerBody>
            <DrawerFooter>
              <Button onClick={onClose}>Close Drawer</Button>
            </DrawerFooter>
          </DrawerContent>
        </Drawer>

        <Menu>
          <MenuButton as={Button} colorScheme="blue" mt={4}>
            Open Menu
          </MenuButton>
          <MenuList>
            <MenuItem>Menu Item 1</MenuItem>
            <MenuDivider />
            <MenuItem>Menu Item 2</MenuItem>
          </MenuList>
        </Menu>

        <Modal isOpen={isOpen} onClose={onClose}>
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>Modal Header</ModalHeader>
            <ModalCloseButton />
            <ModalBody>Modal content...</ModalBody>
            <ModalFooter>
              <Button onClick={onClose}>Close Modal</Button>
            </ModalFooter>
          </ModalContent>
        </Modal>

        <Popover>
          <PopoverTrigger>
            <Button mt={4}>Open Popover</Button>
          </PopoverTrigger>
          <PopoverContent>
            <PopoverHeader>Popover Header</PopoverHeader>
            <PopoverBody>Popover Body</PopoverBody>
          </PopoverContent>
        </Popover>

        <Tooltip label="Tooltip text" placement="top">
          <Button mt={4}>Hover me</Button>
        </Tooltip>
      </Box>

      {/* Media and Icons */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Media and Icons
        </Heading>
        <Avatar name="John Doe" src="https://bit.ly/ryan-florence" />
        <Icon as={FaBeer} w={8} h={8} color="orange.400" mt={4} />
        <Image
          src="https://via.placeholder.com/150"
          alt="Example Image"
          mt={4}
        />
      </Box>

      {/* Data Display */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Data Display
        </Heading>
        <Badge colorScheme="green">Active</Badge>
        <Card>Card content</Card>
        <Code mt={4}>console.log("Code block");</Code>
        <Divider mt={4} />
        <Kbd>Ctrl + C</Kbd>
        <List spacing={3} mt={4}>
          <ListItem>List Item 1</ListItem>
          <ListItem>List Item 2</ListItem>
        </List>
        <Stat>
          <StatLabel>Stat Label</StatLabel>
          <StatNumber>42</StatNumber>
        </Stat>
        <Table variant="simple" mt={4}>
          <Thead>
            <Tr>
              <Th>Header 1</Th>
              <Th>Header 2</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td>Data 1</Td>
              <Td>Data 2</Td>
            </Tr>
          </Tbody>
        </Table>
        <Tag colorScheme="blue" mt={4}>
          Tag
        </Tag>
      </Box>

      {/* Form Components */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Form Components
        </Heading>
        <FormControl mb={4}>
          <FormLabel>Input</FormLabel>
          <Input placeholder="Type here..." />
        </FormControl>
        <Checkbox>Checkbox</Checkbox>
        <Editable defaultValue="Editable Text" mt={4}>
          <EditablePreview />
          <EditableInput />
        </Editable>
        <NumberInput mt={4}>
          <NumberInputField />
        </NumberInput>
        <Radio mt={4}>Radio</Radio>
        <RangeSlider defaultValue={[10, 30]} mt={4} />
        <Select mt={4}>
          <option>Option 1</option>
        </Select>
        <Slider mt={4} defaultValue={40} />
        <Switch mt={4} />
        <Textarea mt={4} placeholder="Textarea content..." />
        <IconButton mt={4} aria-label="Icon button" icon={<FaBeer />} />
      </Box>

      {/* Navigation */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Navigation
        </Heading>
        <Breadcrumb>
          <BreadcrumbItem>
            <BreadcrumbLink href="#">Home</BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbItem isCurrentPage>
            <BreadcrumbLink href="#">About</BreadcrumbLink>
          </BreadcrumbItem>
        </Breadcrumb>
        <Link href="#" color="teal.500">
          Link
        </Link>
        <SkipNavLink mt={4}>Skip Navigation</SkipNavLink>
      </Box>

      {/* Typography */}
      <Box borderWidth="1px" borderRadius="lg" p={4} mb={4}>
        <Heading size="md" mb={4}>
          Typography
        </Heading>
        <Heading size="lg">Large Heading</Heading>
        <Highlight
          query="highlighted"
          styles={{ px: "2", py: "1", bg: "yellow.200" }}
        >
          This is a highlighted text example
        </Highlight>
        <Text mt={4}>Regular text content.</Text>
      </Box>
    </Box>
  );
};

export default Demo;
