import React from 'react';
import { Image, Box } from '@mantine/core';

/**
 * Props for the ProfilePicture component.
 */
interface ProfilePictureProps {
  src: string;
  alt: string;
}

/**
 * ProfilePicture component that displays a user's profile picture.
 * @param {ProfilePictureProps} props - The props for the ProfilePicture component.
 * @param {string} props.src - The source URL of the profile picture.
 * @param {string} props.alt - The alt text for the profile picture.
 * @returns {JSX.Element} The ProfilePicture component.
 */
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