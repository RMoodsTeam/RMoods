import { jwtDecode } from 'jwt-decode';
import { JwtClaims } from '../rmoods/jwt.ts';
import { RMoodsClient } from '../rmoods/client/RMoodsClient.ts';
import { User } from '../rmoods/types.ts';
import { useQuery } from '@tanstack/react-query';
import Cookies from 'js-cookie';
import { useSetAtom } from 'jotai';
import { userInfoAtom } from '../atoms.ts';
import { Loader } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { MainFallback } from '../routes/MainFallback.tsx';

/**
 * Sets values for values needed in many places in the app
 */
const RMoodsProvider = ({ children }) => {
  const jwt = Cookies.get('RMOODS_JWT');
  const claims = jwtDecode<JwtClaims>(jwt!);

  const {
    data: userInfo,
    error,
    isLoading,
  } = useQuery<User, Error>({
    queryKey: ['userData'],
    queryFn: () => RMoodsClient.getUserInfo(claims.userInfo.id),
  });
  const setUserInfoAtom = useSetAtom(userInfoAtom);

  if (isLoading) {
    return <Loader />;
  }

  if (!jwt) {
    throw new Error('No JWT token found');
  }

  if (!userInfo) {
    throw new Error('Failed to fetch user data: No such user');
  }

  if (error) {
    throw new Error('Failed to fetch user data: ' + error.message);
  }

  setUserInfoAtom(userInfo);

  return children;
};

export default function ({ children }) {
  return (
    <ErrorBoundary FallbackComponent={MainFallback}>
      <RMoodsProvider>{children}</RMoodsProvider>
    </ErrorBoundary>
  );
}
