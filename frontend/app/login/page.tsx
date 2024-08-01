"use client";
import React, { useState } from "react";
import { useGoogleLogin } from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import { useRouter } from "next/navigation";
import { useAtom, atom } from "jotai";
import Cookies from "js-cookie";

export const userInfoAtom = atom<any>({});

export async function postGoogleCode(codeResponse: { code: string }) {
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
  return answer.user_info;
}

export default function LoginPage() {
  const router = useRouter();
  const [_, setUserInfo] = useAtom(userInfoAtom);
  const googleLogin = useGoogleLogin({
    onSuccess: async (codeResponse) => {
      const info = await postGoogleCode(codeResponse);
      setUserInfo(info);
      router.refresh();
      router.replace("/dashboard");
    },
    flow: "auth-code",
  });
  1;
  return (
    <div>
      <h1>Log in</h1>
      <GoogleSignInButton onClick={() => googleLogin()} />
    </div>
  );
}
