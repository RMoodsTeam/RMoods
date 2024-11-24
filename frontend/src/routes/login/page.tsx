import {useGoogleLogin} from "@react-oauth/google";
import GoogleSignInButton from "./GoogleSignInButton";
import {useAtom} from "jotai";
import Cookies from "js-cookie";
import {userInfoAtom} from "../../atoms";
import {useNavigate} from "react-router-dom";
import {
  Center,
  Text,
  Paper,
  Stack,
  Title,
  Box,
} from "@mantine/core";


/**
 * Login card with Google sign in button
 * @returns Element
 */
const LoginCard = () => {
  const navigate = useNavigate();
  const postGoogleCode = async (codeResponse: { code: string }) => {
    // for test purposes it will stay at this URL for now
    const url = 'http://localhost:8001/auth/login';
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
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
      Cookies.set('RMOODS_JWT', res.jwt, { expires: 30 });
      navigate('/dashboard');
    },
    flow: 'auth-code',
  });

  return (
    <Center p="xl" h="70vh">
      <Paper radius="md" w="540" p="xl" withBorder shadow="md">
        <Stack gap="md" align="center">
          <Box>
            <Title order={1} ta="center">
              Welcome to RMoods!
            </Title>
          </Box>
          <GoogleSignInButton onClick={googleLogin} />
        </Stack>
      </Paper>
    </Center>
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
