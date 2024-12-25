import { useNavigate } from 'react-router-dom';
import { Avatar, Menu } from '@mantine/core';
import { changeDefaultGoogleProfilePictureSize } from '../../../../utility/changeDefaultGoogleProfilePictureSize.ts';
import { ErrorBoundary } from 'react-error-boundary';
import { UserMenuFallback } from './UserMenuFallback.tsx';
import { logout } from '../../../../utility/logout.ts';
import classes from './UserMenu.module.scss';
import { useAtomValue } from 'jotai';
import { userInfoAtom } from '../../../../atoms.ts';

/**
 * UserMenu component that displays the user menu.
 * @returns {JSX.Element} The UserMenu component.
 */
const UserMenu = () => {
  const navigate = useNavigate();

  const userInfo = useAtomValue(userInfoAtom);

  const size = 45;
  const resizedPicture = userInfo?.picture
    ? changeDefaultGoogleProfilePictureSize(userInfo.picture, size)
    : '';

  return (
    <Menu id="user-dropdown">
      <Menu.Target>
        <Avatar
          src={resizedPicture}
          radius="xl"
          size={size}
          className={classes.avatar}
          imageProps={{ referrerPolicy: 'no-referrer' }}
        />
      </Menu.Target>
      <Menu.Dropdown>
        <Menu.Item onClick={() => navigate('/user')}>Profile</Menu.Item>
        <Menu.Item onClick={() => navigate('/dashboard')}>Dashboard</Menu.Item>
        <Menu.Item onClick={() => navigate('/settings')}>Settings</Menu.Item>
        <Menu.Item onClick={() => logout(navigate)}>Log out</Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
};

export default function () {
  const navigate = useNavigate();
  return (
    <ErrorBoundary
      FallbackComponent={UserMenuFallback}
      onReset={() => {
        logout(navigate);
      }}
    >
      <UserMenu />
    </ErrorBoundary>
  );
}
