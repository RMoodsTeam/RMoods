//import { userInfoFromJWT } from "../../utility/serverActions";

import {Avatar, Box, Card, Title} from "@mantine/core";

const LoggedUserInfo = async () => {
  // const userInfo = await userInfoFromJWT();
  const userInfo = {
    name: "John Doe",
    email: "",
    picture: "https://bit.ly/dan-abramov",
  };

  return (
    <Card>
      <Box>
        <Title order={3}>User Info</Title>
      </Box>
      <Box>
        <Avatar src={userInfo.picture} size="2xl"/>
      </Box>
      <Box>
        <Box>
          <label>Name</label>
          <p>{userInfo.name}</p>
        </Box>
        <Box>
          <label>Email</label>
          <p>{userInfo.email}</p>
        </Box>
      </Box>
    </Card>
  );
};

export default LoggedUserInfo;
