import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';
import PageTitle from '../../../components/PageTitle.tsx';

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
      <PageTitle title="User Browser" />
      <BrowseUsersPage />
    </ErrorBoundary>
  );
}
