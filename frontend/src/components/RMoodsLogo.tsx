import React from 'react';
import { Box, useMantineColorScheme, Text, Group, Image } from '@mantine/core';

interface RMoodsLogoProps {
  width?: number;
  height?: number;
}

const RMoodsLogo = ({ width = 60, height = 80 }: RMoodsLogoProps) => {
  const { colorScheme } = useMantineColorScheme();
  const isDark = colorScheme === 'dark';

  return (
    <Group align="center" gap="xs">
      <Box w={width} h={height}>
        <Image
          src="/src/assets/rmoods-logo.svg"
          alt="RMoods Logo"
          w="100%"
          h="100%"
          fit="contain"
        />
      </Box>
      <Text
        size="24px"
        fw={700}
        variant="gradient"
        gradient={{ from: '#FF6B2B', to: '#246B61', deg: 90 }}
      >
        r/moods
      </Text>
    </Group>
  );
};

export default RMoodsLogo;
export { RMoodsLogo };
