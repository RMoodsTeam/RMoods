import {Outlet} from "react-router-dom";
import Navbar from "./components/navbar/Navbar";
import NoSidebarLayout from "./components/NoSidebarLayout.tsx";
import Footer from "./components/Footer";
import {Flex} from "@chakra-ui/react";

const Layout = () => {
  return (
    <Flex
      style={{
        minHeight: "100vh",
        flexDirection: "column",
      }}
    >
      <Navbar/>
      <Outlet/>
      <Footer/>
    </Flex>
  );
};

export default Layout;
