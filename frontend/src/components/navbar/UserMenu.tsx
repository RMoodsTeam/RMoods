import Cookies from 'js-cookie';
import { useNavigate } from 'react-router-dom';
import { Avatar, Menu } from '@mantine/core';
import authFetch from '../../rmoods/client/authFetch.ts';
import { useEffect, useState } from 'react';
import { jwtDecode } from 'jwt-decode';
import { JwtClaims } from '../../rmoods/jwt.ts';
import { changeDefaultGoogleProfilePictureSize } from '../../utility/changeDefaultGoogleProfilePictureSize.ts';
import { ErrorBoundary } from 'react-error-boundary';
import { UserMenuFallback } from '../fallbacks/UserMenuFallback.tsx';

/**
 * User interface representing the user data.
 */
interface User {
  name: string;
  picture: string;
}

/**
 * UserMenu component that displays the user menu.
 * @returns {JSX.Element} The UserMenu component.
 */
const UserMenu = () => {
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const token = Cookies.get('RMOODS_JWT');
    if (!token) {
      navigate('/login');
      return;
    }

    let isMounted = true;
    try {
      const data = jwtDecode<JwtClaims>(token);
      const id = data.userInfo.id;

      authFetch('http://localhost:8001/api/user?id=' + id).then((response) => {
        response.json().then((data) => {
          if (isMounted) {
            setUser(data);
            setLoading(false);
          }
        });
      });
    } catch (error) {
      console.error('JWT token could not be decoded.', error);
      if (isMounted) {
        setLoading(false);
      }
    }
    return () => {
      isMounted = false;
    };
  }, [navigate]);

  const handleLogout = () => {
    console.log('Logging out');
    Cookies.remove('RMOODS_JWT');
    navigate('/login');
  };

  const size = 45;
  const resizedPicture = user
    ? changeDefaultGoogleProfilePictureSize(user.picture, size)
    : '';
  return (
    <Menu id="user-dropdown">
      <Menu.Target>
        <Avatar
          src={resizedPicture}
          radius="xl"
          size={size}
          style={{ cursor: 'pointer' }}
          imageProps={{ referrerPolicy: 'no-referrer' }}
        />
      </Menu.Target>
      <Menu.Dropdown>
        <Menu.Item onClick={() => navigate('/user')}>Profile</Menu.Item>
        <Menu.Item onClick={() => navigate('/dashboard')}>Dashboard</Menu.Item>
        <Menu.Item onClick={() => navigate('/settings')}>Settings</Menu.Item>
        <Menu.Item onClick={() => handleLogout()}>Log out</Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={UserMenuFallback}>
      <UserMenu />
    </ErrorBoundary>
  );
}
