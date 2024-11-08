import {Card} from "@mantine/core";

const flexFooter = {
  borderRadius: 0,
  height: "10vh",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  margin: 0,
  marginTop: "auto",
};

/**
 * Footer component rendering authors credentials
 * @returns Element
 */
const Footer = () => {
  return (
    <Card style={{...flexFooter}}>
      <footer>RMoods</footer>
    </Card>
  );
};

export default Footer;
