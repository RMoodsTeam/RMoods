import Link from "next/link";
/**
 * Home component, returns main content
 * @returns {TSX}
 */
export default function Home() {
  return (
    <div>
      <h1>RMoods</h1>
      <Link href={"/login"}>Go to Login</Link>
    </div>
  );
}
