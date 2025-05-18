import { Box, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';
import PageTitle from '../../../components/PageTitle.tsx';

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
      <PageTitle title="About" />
      <ThesisPage />
    </ErrorBoundary>
  );
}
