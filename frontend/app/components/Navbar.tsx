import React from "react";
import { Box, Card, Link } from "@mui/joy";
import UserMenu from "./UserMenu";
import ThemeSwitch from "./ThemeSwitch";

const gridNav = {
  display: "grid",
  gridTemplateColumns: "1fr 1fr 1fr",
  gap: "1rem",
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
    <Box sx={{ display: "flex", justifyContent: "right" }}>
      <ThemeSwitch />
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
