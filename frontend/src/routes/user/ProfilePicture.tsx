import React from 'react';
import { Image, Box } from '@mantine/core';
import { changeDefaultGoogleProfilePictureSize } from '../../utility/changeDefaultGoogleProfilePictureSize.ts';
import { ErrorBoundary } from 'react-error-boundary';
import { ProfilePictureFallback } from '../../components/fallbacks/ProfilePictureFallback.tsx';

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
  const resizedSrc = changeDefaultGoogleProfilePictureSize(src, 250);

  return (
    <Box
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      }}
    >
      <Image
        src={resizedSrc}
        alt={alt}
        w={'250px'}
        h={'250px'}
        fit="contain"
        radius="50%"
        referrerPolicy="no-referrer"
        style={{ objectFit: 'cover', display: 'block' }}
      />
    </Box>
  );
};

export default function ({ src, alt }) {
  return (
    <ErrorBoundary FallbackComponent={ProfilePictureFallback}>
      <ProfilePicture src={src} alt={alt} />
    </ErrorBoundary>
  );
}
