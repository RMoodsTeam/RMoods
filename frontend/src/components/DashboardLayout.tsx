import {Box, Flex} from "@chakra-ui/react";
import Sidebar from "./Sidebar";
import {Outlet} from "react-router-dom";
import Navbar from "./navbar/Navbar.tsx";
import Footer from "./Footer.tsx";

const dashboardFlex = {
  flex: "auto",
  flexDirection: "column",
}

const dashboardContainer = {
  marginX: '5%',
  marginY: '2em'
}

const DashboardLayout = () => {
  return (
    <Flex>
      <Sidebar/>
      <Flex
        sx={dashboardFlex}
      >
        <Navbar/>
        <Box sx={dashboardContainer}>
          <Outlet/>
        </Box>
        <Footer/>
      </Flex>
    </Flex>
  );
};

export default DashboardLayout;
