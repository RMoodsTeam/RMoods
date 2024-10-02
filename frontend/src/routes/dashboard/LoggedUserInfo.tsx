import { Avatar, Box, Card, Heading } from "@chakra-ui/react";
import { userInfoFromJWT } from "../../utility/serverActions";

const LoggedUserInfo = async () => {
  const userInfo = await userInfoFromJWT();
  return (
    <Card>
      <Box>
        <Heading as="h3">User Info</Heading>
      </Box>
      <Box>
        <Avatar src={userInfo.picture} size="2xl" />
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
