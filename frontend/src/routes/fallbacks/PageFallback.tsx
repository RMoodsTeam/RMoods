import { Center, Stack, Title, Text, Paper, Card } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';

export const PageFallback = ({ error }) => {
  console.error('PageFallback:', error.message);
  return (
    <Center>
      <Card shadow="xs" p="xl" withBorder radius={20}>
        <Stack>
          <Center>
            <IconAlertCircle size={48} color="red" />
          </Center>
          <Center>
            <Title order={2}>Oops! Something went wrong</Title>
          </Center>
          <Center>
            <Text>
              The page encountered an error and couldn't be displayed.
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
