import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';

const BrowseReportsPage = () => {
  return (
    <Box>
      <Title order={1}>Browse Reports</Title>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <BrowseReportsPage />
    </ErrorBoundary>
  );
}
