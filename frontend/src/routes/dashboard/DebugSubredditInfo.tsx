import { Card, Heading } from "@chakra-ui/react";
//import { fetchSubredditInfo } from "../../rmoods/api";

/**
 * Async debug function to display subreddit info
 * @returns Promise<Element>
 */
export default async function DebugSubredditInfo() {
  //const subredditData = await fetchSubredditInfo("Polska");
  const subredditData = { test: "test" };
  return (
    <Card>
      <Heading as="h3">Debug subreddit info</Heading>
      <div>{JSON.stringify(subredditData, null, 2)}</div>
    </Card>
  );
}
