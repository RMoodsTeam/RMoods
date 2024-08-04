import React from "react";
import DebugRedditUserInfo from "./DebugUserInfo";
import LoggedUserInfo from "./LoggedUserInfo";
import DebugSubredditInfo from "./DebugSubredditInfo";

const Dashboard = async () => {
  return (
    <>
      <LoggedUserInfo />
      <DebugRedditUserInfo />
      <DebugSubredditInfo />
    </>
  );
};

export default Dashboard;
