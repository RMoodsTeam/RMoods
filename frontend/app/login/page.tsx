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

export default function LoginPage() {
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
