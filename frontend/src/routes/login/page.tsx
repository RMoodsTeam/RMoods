import { useGoogleLogin } from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import { useAtom } from "jotai";
import Cookies from "js-cookie";
import { userInfoAtom } from "../../atoms";
import { Card, Center, Heading } from "@chakra-ui/react";
import { useNavigate } from "react-router-dom";

/**
 * Login card with Google sign in button
 * @returns Element
 */
const LoginCard = () => {
  const navigate = useNavigate();
  const postGoogleCode = async (codeResponse: { code: string }) => {
    // for test purposes it will stay at this URL for now
    const url = "http://localhost:8001/auth/login";
    const response = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ code: codeResponse.code }),
    });
    const answer: { jwt: string; user_info: Object } = await response.json();
    console.log(answer);
    return answer;
  };

  const [, setUserInfo] = useAtom(userInfoAtom);
  const googleLogin = useGoogleLogin({
    onSuccess: async (codeResponse) => {
      const res = await postGoogleCode(codeResponse);
      setUserInfo(res.user_info);
      Cookies.set("RMOODS_JWT", res.jwt, { expires: 30 });
      navigate("/dashboard");
    },
    flow: "auth-code",
  });

  return (
    <Card>
      <Heading as="h1" id="login-title">
        Sign in to RMoods
      </Heading>
      <GoogleSignInButton onClick={googleLogin} />
    </Card>
  );
};

/**
 * Login page
 * @returns Element
 */
const Login = () => {
  return (
    <Center>
      <LoginCard />
    </Center>
  );
};

export default Login;
