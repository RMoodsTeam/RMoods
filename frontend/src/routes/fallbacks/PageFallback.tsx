import { Center } from '@mantine/core';

export const PageFallback = ({ error }) => {
  return (
    <Center role="alert">
      <p>Something went wrong:</p>
      <pre style={{ color: 'red' }}>{error.message}</pre>
    </Center>
  );
};
