import UserMenu from "./UserMenu";
//import ThemeSwitch from "./ThemeSwitch";
import { Box, Card, Link } from "@chakra-ui/react";

const gridNav = {
  display: "grid",
  gridTemplateColumns: "1fr 1fr 1fr",
  gap: "1rem",
  margin: "0",
};

const LeftNavItems = () => {
  return (
    <Box sx={{ verticalAlign: "center", display: "flex", gap: 10 }}>
      <Link href="/">Main</Link>
      <Link href="/about">About</Link>
    </Box>
  );
};

const RightNavItems = () => {
  return (
    <Box sx={{ display: "flex", justifyContent: "right", gap: 10 }}>
      {/* <ThemeSwitch /> */}
      <UserMenu />
    </Box>
  );
};

/**
 * Navbar contains components to navigate the website
 */
const Navbar = () => {
  return (
    <Box
      sx={{
        width: "100%",
      }}
    >
      <nav>
        <Card sx={{ ...gridNav, borderRadius: 0, padding: 1 }}>
          <LeftNavItems />
          <div />
          <RightNavItems />
        </Card>
      </nav>
    </Box>
  );
};

export default Navbar;
