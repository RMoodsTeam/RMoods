"use client";
import React from "react";
import Title from "../components/Title";
import { useAtomValue } from "jotai";
import { userInfoAtom } from "../atoms";

const LoggedUserInfo = () => {
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

export default LoggedUserInfo;
