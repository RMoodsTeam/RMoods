import { useGoogleLogin } from '@react-oauth/google';
import GoogleSignInButton from './GoogleSignInButton';
import { useSetAtom } from 'jotai';
import Cookies from 'js-cookie';
import { userInfoAtom } from '../../atoms';
import { useNavigate } from 'react-router-dom';
import { Center, Paper, Stack, Title, Box, Card } from '@mantine/core';
import { notifications } from '@mantine/notifications';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../fallbacks/PageFallback.tsx';

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

    if (response.status !== 200) {
      throw new Error(
        'Authentication failed. Received status code: ' + response.status
      );
    }

    const responseJson = await response.json();
    if (!responseJson?.jwt) {
      throw new Error('Authentication failed. No JWT token received.');
    }

    return responseJson;
  };

  const setUserInfo = useSetAtom(userInfoAtom);
  const googleLogin = useGoogleLogin({
    onSuccess: async (codeResponse) => {
      try {
        const res = await postGoogleCode(codeResponse);
        setUserInfo(res.user_info);
        Cookies.set('RMOODS_JWT', res.jwt, { expires: 30 });
        navigate('/dashboard');
      } catch (error) {
        notifications.show({
          title: 'Login Failed',
          message: 'Authentication failed. Please try again.',
          color: 'red',
        });
        console.error('Login error:', error);
      }
    },
    flow: 'auth-code',
  });

  return (
    <Card radius="md" p="xl" withBorder shadow="md">
      <Stack gap="md" align="center">
        <Box>
          <Title order={1} ta="center">
            Welcome to RMoods!
          </Title>
        </Box>
        <GoogleSignInButton onClick={googleLogin} />
      </Stack>
    </Card>
  );
};

/**
 * Login page
 * @returns Element
 */
const Login = () => {
  return (
    <Center flex={1}>
      <LoginCard />
    </Center>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <Login />
    </ErrorBoundary>
  );
}
