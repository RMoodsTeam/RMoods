import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';

const SandboxPage = () => {
  return (
    <Box>
      <Title order={1}>NLP Sandbox</Title>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <SandboxPage />
    </ErrorBoundary>
  );
}
