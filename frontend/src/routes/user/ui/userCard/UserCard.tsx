import React from 'react';
import { Box, Divider, Flex, Text, Title } from '@mantine/core';
import ProfilePicture from '../profilePicture/ProfilePicture.tsx';
import { User } from '../../page.tsx';
import classes from './UserCard.module.scss';

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
    <Box className={classes.outer}>
      <Flex className={classes.wrapper}>
        <ProfilePicture src={user.picture} alt={user.name} />
        <Title order={2} className={classes.title}>
          {user.name}
        </Title>
        <Text size="sm" c="dimmed" className={classes.text}>
          {user.given_name}
        </Text>
        <Divider my="sm" />
        <Text size="sm" c="dimmed" className={classes.text}>
          {user.email}
        </Text>
      </Flex>
    </Box>
  );
};

export default UserCard;
