import React from 'react';
import { Box, useMantineColorScheme, Text, Group } from '@mantine/core';

interface RMoodsLogoProps {
  width?: number;
  height?: number;
}

const RMoodsLogo = ({ width = 100, height = 80 }: RMoodsLogoProps) => {
  const { colorScheme } = useMantineColorScheme();
  const isDark = colorScheme === 'dark';

  return (
    <Group align="center" gap="xs">
      <Box>
        <svg width={width} height={height} viewBox="0 0 200 160">
          <defs>
            <filter id="shadow" x="-20%" y="-20%" width="140%" height="140%">
              <feDropShadow
                dx="0"
                dy="0"
                stdDeviation="3"
                floodColor="rgba(0, 0, 0, 0.3)"
              />
            </filter>
          </defs>
          <circle cx="45" cy="50" r="25" fill="#FF6B2B" />
          <circle cx="155" cy="50" r="25" fill="#246B61" />
          <ellipse
            cx="40"
            cy="35"
            rx="10"
            ry="6"
            fill="#FF9C6B"
            transform="rotate(-20 40 35)"
          />
          <ellipse
            cx="100"
            cy="80"
            rx="70"
            ry="55"
            fill="#ffffff"
            filter="url(#shadow)"
          />
          <path
            d="M 100,25
               A 70,55 0 1 1 100,135
               A 50,55 0 0 0 100,25"
            fill="#8CCDC3"
          />
          <circle cx="100" cy="84" r="25" fill="#000000" />
          <circle cx="100" cy="69" r="33" fill="#ffffff" />
          <circle cx="70" cy="70" r="9" fill="#000000" />
          <circle cx="130" cy="70" r="9" fill="#000000" />
        </svg>
      </Box>
      <Text
        size="24px"
        fw={700}
        variant="gradient"
        gradient={{ from: '#FF6B2B', to: '#246B61', deg: 90 }}
        mt={-10}
      >
        r/moods
      </Text>
    </Group>
  );
};

export default RMoodsLogo;
export { RMoodsLogo };