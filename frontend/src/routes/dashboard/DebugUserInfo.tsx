import { Box, Card, Heading } from "@chakra-ui/react";
import { fetchUserInfo } from "../../rmoods/api";

/**
 * Astnc debug function to display user info
 * @returns Promise<Element>
 */
export default async function DebugUserInfo() {
  const debugUserInfo = await fetchUserInfo("spez");
  return (
    <Card>
      <Heading as="h3">Debug user info</Heading>
      <Box>{JSON.stringify(debugUserInfo, null, 2)}</Box>
    </Card>
  );
}
