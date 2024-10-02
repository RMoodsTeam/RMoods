import { Card, Typography } from "@mui/joy";
import { fetchUserInfo } from "../rmoods/api";

/**
 * Astnc debug function to display user info
 * @returns Promise<Element>
 */
export default async function DebugUserInfo() {
  const debugUserInfo = await fetchUserInfo("spez");
  return (
    <Card sx={{ wordWrap: "break-word", width: 500 }}>
      <Typography level="h3">Debug user info</Typography>
      <div>{JSON.stringify(debugUserInfo, null, 2)}</div>
    </Card>
  );
}
