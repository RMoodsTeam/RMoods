import React from 'react';
import { Image, Box } from '@mantine/core';

interface ProfilePictureProps {
  src: string;
  alt: string;
}

const ProfilePicture: React.FC<ProfilePictureProps> = ({ src, alt }) => {
  return (
    <Box style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
      <Image
        src={src}
        alt={alt}
        w={'250px'}
        h={'250px'}
        fit="contain"
        radius="50%"
        style={{ objectFit: 'cover', display: 'block' }}
      />
    </Box>
  );
};

export default ProfilePicture;