import {
  Box,
  Button,
  Checkbox,
  Group,
  TextInput,
  Title,
  Select,
  NumberInput,
  SegmentedControl,
  RadioGroup,
  Radio,
  Stack,
} from '@mantine/core';
import { useForm } from '@mantine/form';

const Report = () => {
  const form = useForm({
    initialValues: {
      name: '',
      resourceType: 'subredditPosts', // or 'posts'
      isPublic: 'true',
      size: 'small',
      customSize: undefined,
      analyses: {
        language: false,
        sentiment: false,
      },
    },
  });

  return (
    <Box>
      <Title order={1}>Create Report</Title>

      <form onSubmit={form.onSubmit((values) => console.log(values))}>
        <TextInput
          label="Report Name"
          placeholder="Enter report name"
          {...form.getInputProps('name')}
        />

        <SegmentedControl
          data={[
            { label: 'Public', value: 'true' },
            { label: 'Private', value: 'false' },
          ]}
          {...form.getInputProps('isPublic')}
        />

        <Radio.Group
          name="sourceType"
          label="Select source type"
          {...form.getInputProps('resourceType')}
        >
          <Stack>
            <Radio value="subredditPosts" label="Subreddit Posts" />
            <Radio value="postComments" label="Post Comments" />
            <Radio value="userComments" label="User Comments" />
          </Stack>
        </Radio.Group>

        <Select
          label="Size"
          data={[
            { value: 'small', label: 'Small (30)' },
            { value: 'medium', label: 'Medium (70)' },
            { value: 'large', label: 'Large (100)' },
            { value: 'custom', label: 'Custom' },
          ]}
          {...form.getInputProps('size')}
        />

        {form.values.size === 'custom' && (
          <NumberInput
            label="Custom Size"
            min={0}
            {...form.getInputProps('customSize')}
          />
        )}

        <Title order={3}>Analyses</Title>
        <Checkbox
          label="Language Analysis"
          {...form.getInputProps('analyses.language', { type: 'checkbox' })}
        />
        <Checkbox
          label="Sentiment Analysis"
          {...form.getInputProps('analyses.sentiment', { type: 'checkbox' })}
        />

        <Group justify="flex-end">
          <Button type="submit">Create Report</Button>
        </Group>
      </form>
    </Box>
  );
};

export default Report;
