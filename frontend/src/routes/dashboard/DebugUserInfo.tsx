//import { fetchUserInfo } from "../../rmoods/api";

import { Box, Card, Title } from '@mantine/core';

/**
 * Async debug function to display user info
 * @returns Promise<Element>
 */
export default async function DebugUserInfo() {
  //const debugUserInfo = await fetchUserInfo("spez");
  const debugUserInfo = { user: 'spez' };
  return (
    <Card>
      <Title order={3}>Debug user info</Title>
      <Box>{JSON.stringify(debugUserInfo, null, 2)}</Box>
    </Card>
  );
}
