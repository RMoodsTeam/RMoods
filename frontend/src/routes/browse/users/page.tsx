import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';

const BrowseUsersPage = () => {
  return (
    <Box>
      <Title order={1}>Browse Users</Title>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <BrowseUsersPage />
    </ErrorBoundary>
  );
}
