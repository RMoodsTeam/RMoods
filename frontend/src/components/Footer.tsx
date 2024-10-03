import { Card } from "@chakra-ui/react";

const flexFooter = {
  borderRadius: 0,
  minHeight: "100px",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  marginTop: "auto",
};

/**
 * Footer component rendering authors credentials
 * @returns Element
 */
const Footer = () => {
  return (
    <Card sx={{ ...flexFooter }}>
      <footer>RMoods</footer>
    </Card>
  );
};

export default Footer;
