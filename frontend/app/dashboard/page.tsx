import React from "react";
import DebugRedditUserInfo from "./DebugUserInfo";
import LoggedUserInfo from "./LoggedUserInfo";
import DebugSubredditInfo from "./DebugSubredditInfo";
import { Typography } from "@mui/joy";

/**
 * Dashboard page, gets user info asynchonously
 * @returns Promise<Element>
 */
const Dashboard = async () => {
  return (
    <>
      <Typography level="h1">Dashboard</Typography>
      <LoggedUserInfo />
      <DebugRedditUserInfo />
      <DebugSubredditInfo />
    </>
  );
};

export default Dashboard;
