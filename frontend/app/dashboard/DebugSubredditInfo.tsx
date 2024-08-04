import Title from "../components/Title";
import { fetchSubredditInfo } from "../rmoods/api";

export default async function DebugSubredditInfo() {
  const subredditData = await fetchSubredditInfo("Polska");
  return (
    <>
      <Title>Debug subreddit info</Title>
      <div>{JSON.stringify(subredditData, null, 2)}</div>
    </>
  );
}
