import { Box, Image } from '@mantine/core';
import React from 'react';

export const ProfilePictureFallback = ({ error }) => {
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
        alt={`defaulting to placeholder: ${error.message}`}
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
