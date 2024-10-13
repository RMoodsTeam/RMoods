import {Flex} from "@chakra-ui/react";
import Sidebar from "./Sidebar";
import {Outlet} from "react-router-dom";

const flexMain = {
  flex: "auto",
  flexDirection: "column",
  justifyContent: "center",
  alignItems: "center",
  padding: "60px 1em 0",
};

/**
 * Main div container for all content.
 * Centers children vertically and horizontally.
 */

const SidebarContainer = () => {
  return (
    <Flex
      sx={{flex: "auto"}}>
      <Sidebar/>
      <Flex
        sx={{
          ...flexMain,

        }}
      >
        <Outlet/>
      </Flex>
    </Flex>
  );
};

export default SidebarContainer;
