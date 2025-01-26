import React from 'react';
import { Flex, Image } from '@mantine/core';
import { changeDefaultGoogleProfilePictureSize } from '../../../../utility/changeDefaultGoogleProfilePictureSize.ts';
import { ErrorBoundary } from 'react-error-boundary';
import { ProfilePictureFallback } from '../profilePictureFallback/ProfilePictureFallback.tsx';
import classes from './ProfilePicture.module.scss';

/**
 * Props for the ProfilePicture component.
 */
interface ProfilePictureProps {
  src: string;
  alt: string;
}

const PICTURE_SIZE = 250;

/**
 * ProfilePicture component that displays a user's profile picture.
 * @param {ProfilePictureProps} props - The props for the ProfilePicture component.
 * @param {string} props.src - The source URL of the profile picture.
 * @param {string} props.alt - The alt text for the profile picture.
 * @returns {JSX.Element} The ProfilePicture component.
 */
const ProfilePicture = ({ src, alt }: ProfilePictureProps) => {
  const resizedSrc = changeDefaultGoogleProfilePictureSize(src, PICTURE_SIZE);

  return (
    <Flex className={classes.flex}>
      <Image
        src={resizedSrc}
        alt={alt}
        w={`${PICTURE_SIZE}px`}
        h={`${PICTURE_SIZE}px`}
        fit="contain"
        radius="50%"
        referrerPolicy="no-referrer"
        className={classes.image}
      />
    </Flex>
  );
};

export default function ({ src, alt }) {
  return (
    <ErrorBoundary FallbackComponent={ProfilePictureFallback}>
      <ProfilePicture src={src} alt={alt} />
    </ErrorBoundary>
  );
}
