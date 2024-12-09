import { Box } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../fallbacks/PageFallback.tsx';

const Settings = () => {
  return <Box>Work in progress</Box>;
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <Settings />
    </ErrorBoundary>
  );
}
