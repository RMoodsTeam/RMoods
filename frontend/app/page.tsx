import React from "react";
import { Button, Card, Link, Typography } from "@mui/joy";

/**
 * Main page with general information about RMoods, found at `/`
 */
export default function Home() {
  return (
    <>
      <Typography level="h1">RMoods</Typography>
      <Link href={"/login"}>Go to Login</Link>
      <Card>
        <div>Hello! This is the main page of RMoods</div>
      </Card>
      <Button variant="solid">Click me</Button>
    </>
  );
}
