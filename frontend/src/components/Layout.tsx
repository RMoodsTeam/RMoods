import { Flex } from "@chakra-ui/react";
import Footer from "./Footer";
import MainContainer from "./MainContainer";
import { Outlet } from "react-router-dom";
import Navbar from "./navbar/Navbar";

const flexLayout = {
  flexDirection: "column",
  minHeight: "100vh",
};

const Layout = () => {
  return (
    <Flex sx={{ ...flexLayout }}>
      <Navbar />
      <MainContainer>
        <Outlet />
      </MainContainer>
      <Footer />
    </Flex>
  );
};

export default Layout;
