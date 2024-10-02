"use client";
import React from "react";
import { useGoogleLogin } from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import { useAtom } from "jotai";
import Cookies from "js-cookie";
import { userInfoAtom } from "../atoms";
import { postGoogleCode, serverRedirect } from "./postGoogleCode";
import { Card, Typography } from "@mui/joy";

/**
 * Login card with Google sign in button
 * @returns Element
 */
const LoginCard = () => {
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
      <Typography level="h1" id="login-title">
        Sign in to RMoods
      </Typography>
      <GoogleSignInButton onClick={() => googleLogin()} />
    </Card>
  );
};

/**
 * Login page
 * @returns Element
 */
const Login = () => {
  return (
    <div className="flex flex-col items-center justify-center">
      <LoginCard />
    </div>
  );
};

export default Login;
