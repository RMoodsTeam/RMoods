import React from "react";
import { userInfoFromJWT } from "../utility/serverActions";
import Image from "next/image";
import { Card, Typography } from "@mui/joy";

const LoggedUserInfo = async () => {
  const userInfo = await userInfoFromJWT();
  return (
    <Card>
      <div>
        <Typography level="h3">User Info</Typography>
      </div>
      <div>
        <Image
          src={userInfo.picture}
          width={100}
          height={100}
          alt="user profile picture"
        />
      </div>
      <div>
        <div>
          <label>Name</label>
          <p>{userInfo.name}</p>
        </div>
        <div>
          <label>Email</label>
          <p>{userInfo.email}</p>
        </div>
      </div>
    </Card>
  );
};

export default LoggedUserInfo;
