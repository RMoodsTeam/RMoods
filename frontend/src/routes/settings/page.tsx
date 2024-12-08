import { Box } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../fallbacks/PageFallback.tsx';

const Settings = () => {
  return (
    <ErrorBoundary fallback={<PageFallback />}>
      <Box>Work in progress</Box>
    </ErrorBoundary>
  );
};

export default Settings;
