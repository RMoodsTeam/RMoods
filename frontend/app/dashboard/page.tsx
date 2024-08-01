"use client";
import { useAtomValue } from "jotai";
import React from "react";
import { userInfoAtom } from "../atoms";
import Title from "../components/Title";

const Dashboard = () => {
  const userInfo = useAtomValue(userInfoAtom);
  return (
    <>
      <Title>Dashboard</Title>
      <p>User Info</p>
      <label>Name</label>
      <p>{userInfo.name}</p>
      <label>Email</label>
      <p>{userInfo.email}</p>
      
    </>
  );
};

export default Dashboard;
