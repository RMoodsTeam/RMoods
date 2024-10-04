import DebugRedditUserInfo from "./DebugUserInfo";
import LoggedUserInfo from "./LoggedUserInfo";
import DebugSubredditInfo from "./DebugSubredditInfo";
import { Heading } from "@chakra-ui/react";
import { Await } from "react-router-dom";

/**
 * Dashboard page, gets user info asynchonously
 * @returns Promise<Element>
 */
const Dashboard = () => {
  return (
    <>
      <Heading as="h1">Dashboard</Heading>
      {/* <LoggedUserInfo />
      <DebugRedditUserInfo />
      <DebugSubredditInfo /> */}
    </>
  );
};

export default Dashboard;
