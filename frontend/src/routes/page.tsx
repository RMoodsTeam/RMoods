import Demo from '../components/Demo';
import { Center, Container, Title } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from './fallbacks/PageFallback.tsx';

const Root = () => {
  return (
    <ErrorBoundary fallback={<PageFallback />}>
      <Container>
        <Center>
          <Title order={1}>RMoods Demo Content</Title>
        </Center>
        <Demo />
      </Container>
    </ErrorBoundary>
  );
};

export default Root;
