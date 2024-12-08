import React from 'react';
import { Box, Divider, Text, Title } from '@mantine/core';
import ProfilePicture from './ProfilePicture';
import { User } from './page.tsx';

/**
 * Props for the UserCard component.
 */
interface UserCardProps {
  user: User;
}

/**
 * UserCard component that displays a user's profile information.
 * @param {UserCardProps} props - The props for the UserCard component.
 * @param {User} props.user - The user data to display.
 * @returns {JSX.Element} The UserCard component.
 */
const UserCard: React.FC<UserCardProps> = ({ user }) => {
  return (
    <Box style={{ width: '350px', height: '600px', padding: '20px' }}>
      <Box
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
        }}
      >
        <ProfilePicture src={user.picture} alt={user.name} />
        <Title order={2} style={{ marginTop: '10px', fontSize: '22px' }}>
          {user.name}
        </Title>
        <Text size='sm' c='dimmed' style={{ fontSize: '18px' }}>
          {user.given_name}
        </Text>
        <Divider my='sm' />
        <Text size='sm' c='dimmed' style={{ fontSize: '18px' }}>
          {user.email}
        </Text>
      </Box>
    </Box>
  );
};

export default UserCard;
