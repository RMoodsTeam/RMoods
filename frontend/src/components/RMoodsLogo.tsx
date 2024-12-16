import React from 'react';
import { Box, useMantineColorScheme } from '@mantine/core';

export const RMoodsLogo = () => {
  const { colorScheme } = useMantineColorScheme();
  const isDark = colorScheme === 'dark';

  return (
    <Box p="md">
      <svg width="200" height="160" viewBox="0 0 200 160">
        {/* Ears behind face */}
        <circle cx="45" cy="40" r="25" fill="#FF6B2B" />
        <circle cx="155" cy="40" r="25" fill="#2B7FFF" />

        {/* Base Mask Shape */}
        <ellipse cx="100" cy="80" rx="70" ry="55" fill="#ffffff" />

        {/* Kształt księżyca */}
        <path
          d="M 100,25
             A 70,55 0 1 1 100,135
             A 50,55 0 0 0 100,25"
          fill="#84C5F4"
        />

        {/* Crescent moon smile - using overlapping circles */}
        <circle cx="100" cy="90" r="25" fill="#000000" />
        <circle cx="100" cy="80" r="28" fill="#ffffff" />

        {/* Eyes on top */}
        <circle cx="75" cy="60" r="8" fill="#000000" />
        <circle cx="125" cy="60" r="8" fill="#000000" />
      </svg>
    </Box>
  );
};

export default RMoodsLogo;
