import React from 'react';
import { Box, Card, Divider, Text, Title } from '@mantine/core';
import ProfilePicture from './ProfilePicture';

interface User {
    name: string;
    givenName: string;
    email: string;
    picture: string;
}

interface UserCardProps {
    user: User;
}

const UserCard: React.FC<UserCardProps> = ({ user }) => {
    return (
        <Box style={{ width: '350px', height: '600px', padding: '20px' }}>
            <Box style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                <ProfilePicture src={user.picture} alt={user.name} />
                <Title order={2} style={{ marginTop: '10px', fontSize: '22px' }}>{user.name}</Title>
                <Text size="sm" c="dimmed" style={{ fontSize: '18px' }}>{user.givenName}</Text>
                <Divider my="sm" />
                <Text size="sm" c="dimmed" style={{ fontSize: '18px' }}>{user.email}</Text>
            </Box>
        </Box>
    );
};

export default UserCard;