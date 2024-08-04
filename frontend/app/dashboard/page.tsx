import React from "react";
import DebugRedditUserInfo from "./DebugUserInfo";
import LoggedUserInfo from "./LoggedUserInfo";
import DebugSubredditInfo from "./DebugSubredditInfo";
import Card from "../components/Card";
import Title from "../components/Title";

const Dashboard = async () => {
  return (
    <>
      <Title>Dashboard</Title>
      <Card>
        <LoggedUserInfo />
      </Card>
      <Card>
        <DebugRedditUserInfo />
      </Card>
      <Card>
        <DebugSubredditInfo />
      </Card>
    </>
  );
};

export default Dashboard;
