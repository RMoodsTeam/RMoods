import { Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../fallbacks/PageFallback.tsx';

/**
 * Dashboard page, gets user info asynchonously
 * @returns Promise<Element>
 */
const Dashboard = () => {
  return (
    <>
      <Title order={1}>Dashboard</Title>
    </>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <Dashboard />
    </ErrorBoundary>
  );
}
