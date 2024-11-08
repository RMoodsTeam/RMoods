//import { fetchSubredditInfo } from "../../rmoods/api";

import {Card, Title} from "@mantine/core";

/**
 * Async debug function to display subreddit info
 * @returns Promise<Element>
 */
export default async function DebugSubredditInfo() {
  //const subredditData = await fetchSubredditInfo("Polska");
  const subredditData = {test: "test"};
  return (
    <Card>
      <Title order={3}>Debug subreddit info</Title>
      <div>{JSON.stringify(subredditData, null, 2)}</div>
    </Card>
  );
}
