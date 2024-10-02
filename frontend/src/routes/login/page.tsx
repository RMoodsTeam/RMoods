"use client";
import { useGoogleLogin } from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import { useAtom } from "jotai";
import Cookies from "js-cookie";
import { userInfoAtom } from "../../atoms";
import { Card, Heading } from "@chakra-ui/react";
//import { postGoogleCode, serverRedirect } from "./postGoogleCode";

/**
 * Login card with Google sign in button
 * @returns Element
 */
const LoginCard = () => {
  // const [, setUserInfo] = useAtom(userInfoAtom);
  // const googleLogin = useGoogleLogin({
  //   onSuccess: async (codeResponse) => {
  //     const res = await postGoogleCode(codeResponse);
  //     setUserInfo(res.user_info);
  //     Cookies.set("RMOODS_JWT", res.jwt, { expires: 30 });
  //     await serverRedirect("/dashboard");
  //   },
  //   flow: "auth-code",
  // });

  return (
    <Card>
      <Heading as="h1" id="login-title">
        Sign in to RMoods
      </Heading>
      <GoogleSignInButton onClick={() => {}} />
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
