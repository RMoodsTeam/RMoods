import { Card, Sheet } from "@mui/joy";
import React from "react";

/**
 * Footer component rendering authors credentials
 * @returns Element
 */
const Footer = () => {
  return (
    <Card
      sx={{
        borderRadius: 0,
        minHeight: "100px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <Sheet>
        <footer>RMoods</footer>
      </Sheet>
    </Card>
  );
};

export default Footer;
