import { jwtDecode } from 'jwt-decode';
import { JwtClaims } from '../rmoods/jwt.ts';
import { RMoodsClient } from '../rmoods/client/RMoodsClient.ts';
import { User } from '../rmoods/types.ts';
import { useQuery } from '@tanstack/react-query';
import Cookies from 'js-cookie';
import { useSetAtom } from 'jotai';
import { userInfoAtom } from '../atoms.ts';
import { Loader } from '@mantine/core';

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

  if (!userInfo) {
    throw new Error('Failed to fetch user data');
  }

  if (!jwt) {
    throw new Error('No JWT token found');
  }

  if (error) {
    throw new Error('Failed to fetch user data');
  }

  setUserInfoAtom(userInfo);

  return children;
};

export default RMoodsProvider;
