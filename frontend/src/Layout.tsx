import { Outlet } from "react-router-dom";
import Navbar from "./components/navbar/Navbar";
import MainContainer from "./components/MainContainer";
import Footer from "./components/Footer";
import { Flex } from "@chakra-ui/react";

const Layout = () => {
  return (
    <Flex
      style={{
        minHeight: "100vh",
        flexDirection: "column",
      }}
    >
      <Navbar />
      <MainContainer>
        <Outlet />
      </MainContainer>
      <Footer />
    </Flex>
  );
};

export default Layout;
