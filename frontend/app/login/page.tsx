"use client";
import React, { useState } from "react";
import { useGoogleLogin } from "@react-oauth/google";
import Cookies from "js-cookie";
import GoogleSignInButton from "./GoogleSignInButton";
import { useRouter } from "next/navigation";
import Title from "../components/Title";
import Card from "../components/Card";

async function postGoogleCode(codeResponse: { code: string }) {
  // for test purposes it will stay at this URL for now
  const url = "http://localhost:8001/auth/login";
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code: codeResponse.code }),
  });
  const answer: { jwt: string; user_info: Object } = await response.json();
  Cookies.set("RMOODS_JWT", answer.jwt, { expires: 30 });
  console.log(answer);
}

export default function LoginPage() {
  const googleLogin = useGoogleLogin({
    onSuccess: (codeResponse) => {
      postGoogleCode(codeResponse);
      router.replace("/dashboard");
    },
    flow: "auth-code",
  });
  const router = useRouter();

  return (
    <div className="flex flex-col items-center justify-center">
      <Card>
        <div className="m-8 flex align-center flex-col">
          <Title className="m-4">Sign in to RMoods</Title>
          <GoogleSignInButton onClick={() => googleLogin()} />
        </div>
      </Card>
    </div>
  );
}
