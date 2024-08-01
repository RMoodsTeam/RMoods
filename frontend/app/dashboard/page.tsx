"use client";
import { useAtomValue } from "jotai";
import React from "react";
import { userInfoAtom } from "../atoms";

const Dashboard = () => {
  const userInfo = useAtomValue(userInfoAtom);
  return (
    <>
      <h1>Dashboard</h1>
      <p>User Info</p>
      <label>Name</label>
      <p>{userInfo.name}</p>
      <label>Email</label>
      <p>{userInfo.email}</p>
    </>
  );
};

export default Dashboard;
