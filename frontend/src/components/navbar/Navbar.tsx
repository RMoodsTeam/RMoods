import { Card, Flex, Grid, Link } from "@chakra-ui/react";
import UserMenu from "./UserMenu";
import ThemeSwitch from "./ThemeSwitch";

const LeftNavItems = () => {
  return (
    <Flex gap={10}>
      <Link href="/">Main</Link>
      <Link href="/about">About</Link>
    </Flex>
  );
};

const RightNavItems = () => {
  return (
    <Flex gap={10} justifyContent={"right"}>
      <ThemeSwitch />
      <UserMenu />
    </Flex>
  );
};

/**
 * Navbar contains components to navigate the website
 */
const Navbar = () => {
  return (
    <Card margin={0}>
      <nav>
        <Grid templateColumns="repeat(3, 1fr)" padding={3}>
          <LeftNavItems />
          <div />
          <RightNavItems />
        </Grid>
      </nav>
    </Card>
  );
};

export default Navbar;
