import { Text } from '@mantine/core';
import Cookies from 'js-cookie';
import { jwtDecode } from "jwt-decode";
import { useEffect, useState } from 'react';
import UserCard from './UserCard';


interface User {
    name: string;
    givenName: string;
    email: string;
    picture: string;
}

const UserPage = () => {
    const [user, setUser] = useState<User | null>(null);

    useEffect(() => {
        const token = Cookies.get("RMOODS_JWT");
        if (!token) {
            console.error("No JWT token found");
            throw new Error("No JWT token found");
        }

        try {
            const decoded: any = jwtDecode(token);
            const userInfo: User = {
                name: decoded.user_info.name,
                givenName: decoded.user_info.given_name,
                email: decoded.user_info.email,
                picture: decoded.user_info.picture,
            };
            setUser(userInfo);
        } catch (error) {
            console.error("JWT token could not be decoded.");
            throw new Error("JWT token could not be decoded.");
        }
    }, []);

    if (!user) {
        return <Text>Loading...</Text>;
    }

    return <UserCard user={user} />;
};

export default UserPage;