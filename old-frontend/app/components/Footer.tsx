import { Card, Sheet } from "@mui/joy";
import React from "react";

const flexFooter = {
  borderRadius: 0,
  minHeight: "100px",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  margin: 0,
};

/**
 * Footer component rendering authors credentials
 * @returns Element
 */
const Footer = () => {
  return (
    <Card sx={{ ...flexFooter }}>
      <Sheet>
        <footer>RMoods</footer>
      </Sheet>
    </Card>
  );
};

export default Footer;
