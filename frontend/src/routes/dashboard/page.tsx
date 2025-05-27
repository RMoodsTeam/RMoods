import { Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';
import PageTitle from '../../components/PageTitle.tsx';

/**
 * Dashboard page, gets user info asynchronously
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
      <PageTitle title="Dashboard" />
      <Dashboard />
    </ErrorBoundary>
  );
}
