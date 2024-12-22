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
import { useMutation } from '@tanstack/react-query';
import { useState } from 'react';
import authFetch from '../../rmoods/client/authFetch';

const fetchNlpResponse = async (text, analysis) => {
  const response = await authFetch(
    `http://localhost:8001/api/playground?analysis=${analysis}`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text }),
    }
  );

  if (!response.ok) {
    throw new Error(`${response.statusText}`);
  }

  return response.json();
};

const NlpSandbox = () => {
  const [text, setText] = useState('');
  const [analysis, setAnalysis] = useState('language');
  const [result, setResult] = useState('');

  const mutation = useMutation({
    mutationFn: (variables: { text: string; analysis: string }) =>
      fetchNlpResponse(variables.text, variables.analysis),
    onSuccess: (data) => setResult(JSON.stringify(data, null, 2)),
    onError: (error) => setResult(`Error: ${error.message}`),
  });

  const handleAnalyze = () => {
    if (!text.trim()) {
      setResult('Error: Input text cannot be empty.');
      return;
    }
    mutation.mutate({ text, analysis });
  };

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
            mb="md"
            value={text}
            onChange={(e) => setText(e.target.value)}
          />
          <Select
            label="Analysis type"
            data={[
              { value: 'language', label: 'Language Detection' },
              { value: 'sentiment', label: 'Sentiment Analysis' },
              { value: 'sarcasm', label: 'Sarcasm Detection' },
              { value: 'spam', label: 'Spam Detection' },
              { value: 'politics', label: 'Political Bias' },
              { value: 'hate_speech', label: 'Hate Speech' },
              { value: 'clickbait', label: 'Clickbait Detection' },
              { value: 'keywords', label: 'Keywords Extraction' },
            ]}
            mb="md"
            value={analysis}
            onChange={(value) => setAnalysis(value || 'language')}
          />
          <Button onClick={handleAnalyze} disabled={mutation.isPending}>
            {mutation.isPending ? 'Analyzing...' : 'Analyze'}
          </Button>
        </Card>
        <Card shadow="sm" p="md">
          <Textarea
            label="Results"
            placeholder="Analysis results will appear here..."
            mb="md"
            readOnly
            value={result}
          />
        </Card>
      </Group>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <NlpSandbox />
    </ErrorBoundary>
  );
}
