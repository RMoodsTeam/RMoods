"use client";
import React from "react";
import { useGoogleLogin } from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import { useAtom } from "jotai";
import Cookies from "js-cookie";
import { userInfoAtom } from "../atoms";
import { postGoogleCode, serverRedirect } from "./postGoogleCode";
import Title from "../components/Title";
import Card from "../components/Card";

function LoginCard() {
  const [, setUserInfo] = useAtom(userInfoAtom);
  const googleLogin = useGoogleLogin({
    onSuccess: async (codeResponse) => {
      const res = await postGoogleCode(codeResponse);
      setUserInfo(res.user_info);
      Cookies.set("RMOODS_JWT", res.jwt, { expires: 30 });
      await serverRedirect("/dashboard");
    },
    flow: "auth-code",
  });
  return (
    <Card>
    <div className="m-8 flex align-center flex-col">
      <Title className="m-4" id='login-title'>Sign in to RMoods</Title>
      <GoogleSignInButton onClick={() => googleLogin()} />
    </div>
  </Card>
  )
}

export default function Login() {
  return (
    <div className="flex flex-col items-center justify-center">
      <LoginCard/>
    </div>
  );
}
