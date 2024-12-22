import {
  Box,
  Button,
  Card,
  Group,
  Select,
  Textarea,
  Title,
} from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';

const SandboxPage = () => {
  return (
    <Box w="100%" p="md">
      <Title order={1} mb="lg">
        NLP Sandbox
      </Title>
      <Group grow align="stretch" h="calc(100% - 60px)">
        <Card shadow="sm" p="md">
          <Textarea
            label="Enter text to analyze"
            placeholder="Type or paste your text here..."
            minRows={4}
            mb="md"
          />
          <Select
            label="Analysis type"
            data={[
              { value: 'language', label: 'Language Detection' },
              { value: 'sentiment', label: 'Sentiment Analysis' },
              { value: 'sarcasm', label: 'Sarcasm Detection' },
              { value: 'spam', label: 'Spam Detection' },
              { value: 'political', label: 'Political Bias' },
              { value: 'hatespeech', label: 'Hate Speech' },
              { value: 'clickbait', label: 'Clickbait Detection' },
              { value: 'keywords', label: 'Keywords Extraction' },
            ]}
            mb="md"
          />
          <Button>Analyze</Button>
        </Card>
        <Card shadow="sm" p="md">
          <Textarea
            label="Results"
            placeholder="Analysis results will appear here..."
            minRows={4}
            mb="md"
            readOnly
          />
        </Card>
      </Group>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <SandboxPage />
    </ErrorBoundary>
  );
}
