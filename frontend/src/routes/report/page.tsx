import {
  Box,
  Button,
  Checkbox,
  Group,
  TextInput,
  Title,
  NumberInput,
  SegmentedControl,
  Radio,
  Stack,
  Input,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { useEffect, useState } from 'react';
import { RowWrapper } from './inputRow.tsx';
import {
  AnalysisTypeSchema,
  DataSource,
  DataSourceSchema,
  FeedKindSchema,
  FeedSortingKindSchema,
  FeedSortingTimeSchema,
} from '../../rmoods/client/types.ts';
import { zodResolver } from 'mantine-form-zod-resolver';
import { z } from 'zod';
import { DataSourceTable } from './tables.tsx';
import { formatJson } from './formatJson.ts';

const formValidationSchema = z.object({
  name: z.string().min(1, { message: 'Name must be longer than 1 character' }),
  resourceType: FeedKindSchema,
  isPublic: z.enum(['true', 'false']),
  size: z.enum(['small', 'medium', 'large', 'custom']),
  customSize: z.number().optional(),
  sortBy: FeedSortingKindSchema,
  time: FeedSortingTimeSchema,
  dataSource: z
    .array(DataSourceSchema)
    .min(1, { message: 'At least 1 data source is required' }),
  analyses: AnalysisTypeSchema,
});

export type formValues = z.infer<typeof formValidationSchema>;

const Report = () => {
  const form = useForm({
    initialValues: {
      name: '',
      resourceType: 'subredditPosts',
      isPublic: 'true',
      size: 'small',
      customSize: undefined,
      sortBy: 'hot',
      time: 'day',
      dataSource: [] as DataSource[],
      analyses: {
        language: false,
        sentiment: false,
        sarcasm: false,
        spam: false,
        politics: false,
        hateSpeech: false,
        clickbait: false,
        trolling: false,
      },
    },
    validate: zodResolver(formValidationSchema),
  });

  const [rows, setRows] = useState<RowWrapper[]>([]); // Array to store all rows

  useEffect(() => {
    form.setFieldValue(
      'dataSource',
      rows.map((row) => row.dataSource)
    );
  }, [rows]);

  const makeRowEditHandler =
    (index: number, property: keyof DataSource) => (event: any) => {
      setRows(
        rows.map((row, i) =>
          i === index
            ? {
                ...row,
                dataSource: {
                  ...row.dataSource,
                  [property]:
                    property === 'share' ? event : event.target?.value,
                },
              }
            : row
        )
      );
    };

  const deleteRow = (id: number) => {
    const newRows = rows.filter((row) => row.id !== id);
    setRows(newRows);
  };

  return (
    <Box>
      <Title order={1}>Create Report</Title>

      <form onSubmit={form.onSubmit(() => {})}>
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
          onClick={() => {
            setRows([]);
          }}
        >
          <Stack>
            <Radio value="subredditPosts" label="Subreddit Posts" />
            <Radio value="postComments" label="Post Comments" />
            <Radio value="userPosts" label="User Posts" />
          </Stack>
        </Radio.Group>

        <Radio.Group
          name="sortBy"
          label="Sort by"
          {...form.getInputProps('sortBy')}
          onClick={() => {
            setRows([]);
          }}
        >
          <Stack>
            <Radio value="hot" label="Hot" />
            <Radio value="new" label="New" />
            <Radio value="rising" label="Rising" />
            <Radio value="top" label="Top" />
            <Radio value="controversial" label="Controversial" />
          </Stack>
        </Radio.Group>

        <Radio.Group
          name="time"
          label="Select time"
          {...form.getInputProps('time')}
          onClick={() => {
            setRows([]);
          }}
        >
          <Stack>
            <Radio value="day" label="Day" />
            <Radio value="week" label="Week" />
            <Radio value="month" label="Month" />
            <Radio value="year" label="Year" />
            <Radio value="all" label="All" />
          </Stack>
        </Radio.Group>

        <Box
          style={(theme) => ({
            border: form.errors.dataSource
              ? `1px solid var(--mantine-color-error)`
              : `1px solid ${theme.colors.gray[7]}`,
            borderRadius: theme.radius.sm,
            padding: theme.spacing.xs,
          })}
        >
          <DataSourceTable
            form={form}
            rows={rows}
            setRows={setRows}
            deleteRow={deleteRow}
            makeRowEditHandler={makeRowEditHandler}
          />
        </Box>
        {form.errors.dataSource && (
          <Input.Error
            style={() => ({
              marginTop: '6px',
            })}
          >
            {form.errors.dataSource}
          </Input.Error>
        )}

        <Radio.Group
          name="size"
          label="Select size"
          {...form.getInputProps('size')}
        >
          <Stack>
            <Radio value="small" label="Small (30)" />
            <Radio value="medium" label="Medium (70)" />
            <Radio value="large" label="Large (100)" />
            <Radio value="custom" label="Custom" />
          </Stack>
        </Radio.Group>
        {form.values.size === 'custom' && (
          <NumberInput
            label="Custom Size"
            min={0}
            {...form.getInputProps('customSize')}
          />
        )}
        <Title order={3}>Analyses</Title>
        <Checkbox
          value={form.values.analyses.language}
          label="Language"
          {...form.getInputProps('analyses.language', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.sentiment}
          label="Sentiment"
          {...form.getInputProps('analyses.sentiment', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.sarcasm}
          label="Sarcasm"
          {...form.getInputProps('analyses.sarcasm', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.spam}
          label="Spam"
          {...form.getInputProps('analyses.spam', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.politics}
          label="Politics"
          {...form.getInputProps('analyses.politics', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.hateSpeech}
          label="Hate Speech"
          {...form.getInputProps('analyses.hateSpeech', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.clickbait}
          label="Clickbait"
          {...form.getInputProps('analyses.clickbait', { type: 'checkbox' })}
        />
        <Checkbox
          value={form.values.analyses.trolling}
          label="Trolling"
          {...form.getInputProps('analyses.trolling', { type: 'checkbox' })}
        />
        <Group justify="flex-end">
          <Button
            type="submit"
            onClick={() => {
              console.log(formatJson(form.values as formValues));
            }}
          >
            Create Report
          </Button>
        </Group>
      </form>
    </Box>
  );
};

export default Report;
