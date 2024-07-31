"use client";
import { useAtom, useAtomValue } from "jotai";
import React from "react";
import { userInfoAtom } from "../login/page";
import Image from "next/image";

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
      <label>Profile picture</label>
    </>
  );
};

export default Dashboard;
