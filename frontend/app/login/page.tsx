"use client";
import React, { useState } from "react";
import { useGoogleLogin } from "@react-oauth/google";
import Cookies from "js-cookie";
import GoogleSignInButton from "./GoogleSignInButton";
import { useRouter } from "next/navigation";

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
    <div>
      <h1>Log in</h1>
      <GoogleSignInButton onClick={() => googleLogin()} />
    </div>
  );
}
