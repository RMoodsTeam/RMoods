import {
  Box,
  Button,
  Checkbox,
  Group,
  Input,
  NumberInput,
  Radio,
  SegmentedControl,
  Stack,
  TextInput,
  Title,
} from '@mantine/core';
import { useForm, UseFormReturnType } from '@mantine/form';
import { useEffect, useState } from 'react';
import { DataSource } from '../../rmoods/client/types.ts';
import { zodResolver } from 'mantine-form-zod-resolver';
import { DataSourceTable } from './tables.tsx';
import {
  ReportFormValidationSchema,
  ReportFormValues,
  RowWrapper,
} from './types.ts';
import { transformJson } from './transformJson.ts';
import { RMoodsClient } from '../../rmoods/client/RMoodsClient.ts';

/**
 * Report component for creating a new report.
 *
 * @returns {JSX.Element} - The rendered report creation form.
 */
const Report = () => {
  const form = useForm<ReportFormValues>({
    initialValues: {
      name: '',
      resourceKind: 'subredditPosts',
      isPublic: 'true',
      size: 'small',
      customSize: undefined,
      sortBy: 'hot',
      time: 'day',
      dataSources: [] as DataSource[],
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
    validate: zodResolver(ReportFormValidationSchema),
  });

  const [rows, setRows] = useState<RowWrapper[]>([]); // Array to store all rows

  useEffect(() => {
    form.setFieldValue(
      'dataSources',
      rows.map((row) => row.dataSource)
    );
  }, [rows]);

  /**
   * Creates a handler function to edit a specific property of a row in the data source.
   *
   * @param {number} index - The index of the row to be edited.
   * @param {keyof DataSource} property - The property of the data source to be updated.
   * @returns {Function} - A function that handles the change event for the specified property.
   */
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

  /**
   * Deletes a row from the list of rows by its ID.
   *
   * @param {number} id - The ID of the row to be deleted.
   * @returns {void}
   */
  const deleteRow = (id: number): void => {
    const newRows = rows.filter((row) => row.id !== id);
    setRows(newRows);
  };

  return (
    <Box>
      <Title order={1}>Create Report</Title>

      <form
        onSubmit={form.onSubmit((values) => {
          void values;
          const validationResult = form.validate();
          if (!validationResult.hasErrors) {
            console.log('Form is valid');
          } else {
            console.log('Form is invalid');
          }
          const transformedValues = transformJson(
            form.values as ReportFormValues
          );
          console.log(transformedValues);

          RMoodsClient.requestReport(transformedValues)
            .then((res: unknown) => {
              console.log(res);
            })
            .catch((err: Error) => {
              console.error(err);
            });
        })}
      >
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
          {...form.getInputProps('resourceKind')}
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

        {form.getInputProps('sortBy').value === 'top' ||
        form.getInputProps('sortBy').value === 'controversial' ? (
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
        ) : (
          <></>
        )}

        <Box>
          <DataSourceTable
            form={form as unknown as UseFormReturnType<ReportFormValues>}
            rows={rows}
            setRows={setRows}
            deleteRow={deleteRow}
            makeRowEditHandler={makeRowEditHandler}
          />
        </Box>
        {form.errors.dataSources && (
          <Input.Error
            style={() => ({
              marginTop: '6px',
            })}
          >
            {form.errors.dataSources}
          </Input.Error>
        )}

        <Radio.Group
          name="size"
          label="Select size"
          {...form.getInputProps('size')}
        >
          <Stack>
            <Radio value="30" label="Small (30)" />
            <Radio value="70" label="Medium (70)" />
            <Radio value="100" label="Large (100)" />
            <Radio value="custom" label="Custom" />
          </Stack>
        </Radio.Group>
        {form.values.size === 'custom' && (
          <NumberInput
            label="Custom Size"
            min={0}
            max={500}
            {...form.getInputProps('size')}
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
            // onClick={() => {
            //   console.log(transformJson(form.values as ReportFormValues));
            // }}
          >
            Create Report
          </Button>
        </Group>
      </form>
    </Box>
  );
};

export default Report;
