import React from "react";
import { userInfoFromJWT } from "../utility/serverActions";
import Image from "next/image";
import Title from "../components/Title";

/**
 * Async function to display logged user info
 * @returns Promise<Element>
 */
const LoggedUserInfo = async () => {
  const userInfo = await userInfoFromJWT();
  return (
    <>
      <div className="flex justify-center">
        <Title>User Info</Title>
      </div>
      <div className="m-4 flex justify-center items-center">
        <Image
          className="rounded-full"
          src={userInfo.picture}
          width={100}
          height={100}
          alt="user profile picture"
        />
      </div>
      <div>
        <div className="grid grid-cols-2">
          <label className="pr-2 text-gray-400">Name</label>
          <p>{userInfo.name}</p>
        </div>
        <div className="grid grid-cols-2">
          <label className="pr-2 text-gray-400">Email</label>
          <p>{userInfo.email}</p>
        </div>
      </div>
    </>
  );
};

export default LoggedUserInfo;
