import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';
import PageTitle from '../../components/PageTitle.tsx';

const Settings = () => {
  return (
    <Box>
      <Title order={1}>Settings</Title>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <PageTitle title="Settings" />
      <Settings />
    </ErrorBoundary>
  );
}
