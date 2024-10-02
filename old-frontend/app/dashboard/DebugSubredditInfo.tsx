import { Card, Typography } from "@mui/joy";
import { fetchSubredditInfo } from "../rmoods/api";

/**
 * Async debug function to display subreddit info
 * @returns Promise<Element>
 */
export default async function DebugSubredditInfo() {
  const subredditData = await fetchSubredditInfo("Polska");
  return (
    <Card>
      <Typography level="h3">Debug subreddit info</Typography>
      <div>{JSON.stringify(subredditData, null, 2)}</div>
    </Card>
  );
}
