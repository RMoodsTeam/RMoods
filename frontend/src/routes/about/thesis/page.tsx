import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';

const ThesisPage = () => {
  return (
    <Box>
      <Title order={1}>Thesis</Title>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <ThesisPage />
    </ErrorBoundary>
  );
}
