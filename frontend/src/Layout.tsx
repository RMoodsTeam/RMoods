import {Outlet} from "react-router-dom";
import Navbar from "./components/navbar/Navbar";
import Footer from "./components/Footer";
import {Flex} from "@mantine/core";

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
