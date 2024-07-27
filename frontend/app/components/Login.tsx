import React, { useState } from "react";
import { useGoogleLogin } from "@react-oauth/google";
import Cookies from "js-cookie";

/**
 * Login component for user to input their credentials
 */

async function postGoogleCode(codeResponse: { code: string }) {
  // for test purposes it will stay at this URL for now
  const url = "http://localhost:8001/auth/auth/login";
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code: codeResponse.code }),
  });
  const answer = await response.json();
  Cookies.set("auth", codeResponse.code, { expires: 30 });
  console.log(answer);
}

const Login = () => {
  const googleLogin = useGoogleLogin({
    onSuccess: (codeResponse) => postGoogleCode(codeResponse),
    flow: "auth-code",
  });

  return (
    <button
      onClick={() => {
        googleLogin();
      }}
    >
      Log in to Google!
    </button>
  );
};

export default Login;
