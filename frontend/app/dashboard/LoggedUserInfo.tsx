import React from "react";
import Title from "../components/Title";
import { userInfoFromJWT } from "../utility/serverActions";
// import { useAtomValue } from "jotai";
// import { userInfoAtom } from "../atoms";

const LoggedUserInfo = async () => {
  const userInfo = await userInfoFromJWT();
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
