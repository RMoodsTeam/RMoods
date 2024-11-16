import React from 'react';
import { Image } from '@mantine/core';

interface ProfilePictureProps {
    src: string;
    alt: string;
}

const ProfilePicture: React.FC<ProfilePictureProps> = ({ src, alt }) => {
    return (
        <Image
            src={src}
            alt={alt}
            w={'100%'}
            fit="contain"
            radius="50%"
            style={{ objectFit: 'cover', display: 'inline-block' }}
        />
    );
};

export default ProfilePicture;