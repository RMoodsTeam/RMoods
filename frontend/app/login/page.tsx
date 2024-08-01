"use client";
import React from "react";
import { useGoogleLogin } from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import { useAtom } from "jotai";
import Cookies from "js-cookie";
import { userInfoAtom } from "../atoms";
import { postGoogleCode, serverRedirect } from "./postGoogleCode";

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
    <div>
      <h1>Log in</h1>
      <GoogleSignInButton onClick={() => googleLogin()} />
    </div>
  );
}
