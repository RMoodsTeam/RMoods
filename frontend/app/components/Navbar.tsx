import React from "react";
import Link from "./Link";
import { Box, Card } from "@mui/joy";
import User from "./User";

const gridNav = {
  display: "grid",
  gridTemplateColumns: "1fr 1fr 1fr",
  gap: "1rem",
};

const LeftNavItems = () => {
  return (
    <Box>
      <Link href="/">Main</Link>
    </Box>
  );
};

const RightNavItems = () => {
  return (
    <Box sx={{ display: "flex", justifyContent: "right" }}>
      <User />
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
        <Card sx={{ ...gridNav, borderRadius: 0 }}>
          <LeftNavItems />
          <div />
          <RightNavItems />
        </Card>
      </nav>
    </Box>
  );
};

export default Navbar;
