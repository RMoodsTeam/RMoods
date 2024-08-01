import React from "react";
import Card from "./components/Card";
import Button from "./components/Button";
import Title from "./components/Title";
import Link from "./components/Link";
/**
 * Home component, returns main content
 * @returns {TSX}
 */
export default function Home() {
  return (
    <>
      <Title>RMoods</Title>
      <Link href={"/login"}>Go to Login</Link>
      <Card>
        <div>Hello! This is the main page of RMoods</div>
      </Card>
      <Button>Click me</Button>
    </>
  );
}
