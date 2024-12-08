import Cookies from 'js-cookie';
import { useNavigate } from 'react-router-dom';
import { Avatar, Menu } from '@mantine/core';
import authFetch from '../../rmoods/client/authFetch.ts';
import { jwtDecode } from 'jwt-decode';
import { JwtClaims } from '../../rmoods/jwt.ts';
import { changeDefaultGoogleProfilePictureSize } from '../../utility/changeDefaultGoogleProfilePictureSize.ts';
import { useQuery } from '@tanstack/react-query';

/**
 * User interface representing the user data.
 */
interface User {
  name: string;
  picture: string;
}

/**
 * Fetches the user data from the server.
 * @returns {Promise<User>} The user data.
 */
const fetchUserData = async (): Promise<User> => {
  const token = Cookies.get('RMOODS_JWT');
  if (!token) {
    throw new Error('No JWT token found');
  }

  const data = jwtDecode<JwtClaims>(token);
  const id = data.userInfo.id;
  const response = await authFetch(`http://localhost:8001/api/user?id=${id}`);
  if (!response.ok) {
    throw new Error('Failed to fetch user data');
  }

  return response.json();
};


/**
 * UserMenu component that displays the user menu.
 * @returns {JSX.Element} The UserMenu component.
 */
const UserMenu = () => {
  const navigate = useNavigate();

  const { data, error } = useQuery<User, Error>({
    queryKey: ['userData'],
    queryFn: fetchUserData,
    refetchInterval: 5000,
  });

  const handleLogout = () => {
    Cookies.remove('RMOODS_JWT');
    navigate('/login');
  };

  if (error) {
    throw new Error('Failed to fetch user data');
  }

  const size = 45;
  const resizedPicture = data?.picture
    ? changeDefaultGoogleProfilePictureSize(data.picture, size)
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

export default UserMenu;
