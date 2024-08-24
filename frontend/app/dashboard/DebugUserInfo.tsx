import Title from "../components/Title";
import { fetchUserInfo } from "../rmoods/api";

/**
 * Astnc debug function to display user info
 * @returns Promise<Element>
 */
export default async function DebugUserInfo() {
  const debugUserInfo = await fetchUserInfo("spez");
  return (
    <>
      <Title>Debug user info</Title>
      <div>{JSON.stringify(debugUserInfo, null, 2)}</div>
    </>
  );
}
