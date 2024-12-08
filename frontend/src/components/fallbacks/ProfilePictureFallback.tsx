import { Box, Image } from '@mantine/core';
import React from 'react';

export const ProfilePictureFallback = () => {
  return (
    <Box
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      }}
    >
      <Image
        src={'https://placehold.co/250x250?text=PfP'}
        alt="default profile picture"
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
