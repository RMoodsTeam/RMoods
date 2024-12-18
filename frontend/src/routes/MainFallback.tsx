import React from 'react';
import { Card, Center, Stack, Text, Title } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';

export const MainFallback = ({ error }) => {
  console.error('MainFallback:', error.message);
  return (
    <Center>
      <Card shadow="xs" p="xl" withBorder radius={20}>
        <Stack>
          <Center>
            <IconAlertCircle size={48} color="red" />
          </Center>
          <Center>
            <Title order={2}>
              Critical failure! Something went seriously wrong
            </Title>
          </Center>
          <Center>
            <Text>
              The app crashed. The devs will be notified and investigate the
              issue.
            </Text>
          </Center>
          <Center>
            <Text size="sm" color="red">
              {error.message}
            </Text>
          </Center>
        </Stack>
      </Card>
    </Center>
  );
};
