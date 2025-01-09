import {
  Box,
  Button,
  Card,
  Center,
  Group,
  JsonInput,
  Select,
  Stack,
  Text,
  Textarea,
  Title,
} from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';
import { useMutation } from '@tanstack/react-query';
import { useState } from 'react';
import authFetch from '../../rmoods/client/authFetch';
import { IconAlertCircle } from '@tabler/icons-react';

const fetchNlpResponse = async (text, analysis) => {
  const response = await authFetch(`/sandbox?analysis=${analysis}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text }),
  });

  if (!response.ok) {
    throw new Error(`${response.statusText}`);
  }

  return response.json();
};

const NlpSandbox = () => {
  const [text, setText] = useState('');
  const [analysis, setAnalysis] = useState('language');
  const [result, setResult] = useState('');
  const [error, setError] = useState<string | null>(null);

  const mutation = useMutation({
    mutationFn: (variables: { text: string; analysis: string }) =>
      fetchNlpResponse(variables.text, variables.analysis),
    onSuccess: (data) => {
      setResult(JSON.stringify(data, null, 2));
      setError(null);
    },
    onError: (error) => setError(error.message),
  });

  const handleAnalyze = () => {
    if (!text.trim()) {
      setError('Input text cannot be empty.');
      return;
    }
    mutation.mutate({ text, analysis });
  };

  return (
    <Box>
      <Stack>
        <Title order={1}>NLP Sandbox</Title>
        <Card>
          <Stack>
            <Text>
              You can try out all of RMoods NLP models here. You'll see raw
              responses, which during report generation are aggregated and
              analyzed to provide a high-level overview of the data you've
              requested!
            </Text>
            <Group grow align="stretch">
              <Stack>
                <Textarea
                  placeholder="Type or paste your text here"
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                  autosize
                  minRows={14}
                  maxRows={14}
                />
                <Select
                  label="Analysis"
                  data={[
                    { value: 'language', label: 'Language Detection' },
                    { value: 'sentiment', label: 'Sentiment Analysis' },
                    { value: 'sarcasm', label: 'Sarcasm Detection' },
                    { value: 'spam', label: 'Spam Detection' },
                    { value: 'politics', label: 'Political Bias' },
                    { value: 'hateSpeech', label: 'Hate Speech' },
                    { value: 'clickbait', label: 'Clickbait Detection' },
                    { value: 'keywords', label: 'Keywords Extraction' },
                  ]}
                  value={analysis}
                  onChange={(value) => setAnalysis(value || 'language')}
                />
                <Button onClick={handleAnalyze} disabled={mutation.isPending}>
                  {mutation.isPending ? 'Analyzing...' : 'Analyze'}
                </Button>
              </Stack>
              {error ? (
                <Center>
                  <Stack>
                    <Center>
                      <IconAlertCircle size={48} color="red" />
                    </Center>
                    <Center>
                      <Text size="sm" color="red">
                        {error}
                      </Text>
                    </Center>
                  </Stack>
                </Center>
              ) : (
                <JsonInput autosize value={result} minRows={20} />
              )}
            </Group>
          </Stack>
        </Card>
      </Stack>
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
