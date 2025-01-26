import Demo from '../components/Demo';
import { Center, Container, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from './PageFallback.tsx';

const Root = () => {
  return (
    <Container>
      <Center>
        <Title order={1}>RMoods Demo Content</Title>
      </Center>
      <Demo />
    </Container>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <Root />
    </ErrorBoundary>
  );
}
