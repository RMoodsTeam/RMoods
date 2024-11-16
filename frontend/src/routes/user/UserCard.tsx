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
        <Card shadow="sm" padding="lg" style={{ maxWidth: '25%', MaxHeight: '100%' }} withBorder>
            <Box style={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start' }}>
                <ProfilePicture src={user.picture} alt={user.name} />
                <Title order={2} style={{ marginTop: '10px', fontSize: '2vw' }}>{user.name}</Title>
                <Text size="sm" c="dimmed" style={{ fontSize: '1.5vw' }}>{user.givenName}</Text>
                <Divider my="sm" />
                <Text size="sm" c="dimmed" style={{ fontSize: '1.5vw' }}>{user.email}</Text>
            </Box>
        </Card>
    );
};

export default UserCard;