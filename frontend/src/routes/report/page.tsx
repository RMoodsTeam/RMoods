import {
  Box,
  Button,
  Card,
  Center,
  Checkbox,
  Group,
  Input,
  NumberInput,
  SegmentedControl,
  Select,
  Stack,
  Text,
  TextInput,
  Title,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { useEffect, useState } from 'react';
import { DataSource } from './schema.ts';
import { zodResolver } from 'mantine-form-zod-resolver';
import { DataSourceTable } from './DataSourceTable.tsx';
import {
  ReportFormValidationSchema,
  ReportFormValues,
  RowWrapper,
} from './schema.ts';
import { transformJson } from './transformJson.ts';
import { RMoodsClient } from '../../rmoods/client/RMoodsClient.ts';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../fallbacks/PageFallback.tsx';
import { IconLock, IconWorld } from '@tabler/icons-react';

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
      size: '30',
      sortBy: 'hot',
      time: null,
      dataSources: [],
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

  const [selectValue, setSelectValue] = useState('30');

  const handleSelectChange = (value) => {
    form.setFieldValue('size', value); // Update the form value
    setSelectValue(value); // Update the local state for conditional rendering
  };

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
    <Stack>
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
        <Stack>
          <Card>
            <Stack>
              <Title order={2}>Title</Title>
              <Text>
                Choose a title for your report. This will help you find it
                later.
              </Text>
              <TextInput
                placeholder="eg. Sentiment on r/AskReddit"
                {...form.getInputProps('name')}
              />
            </Stack>
          </Card>

          <Card>
            <Stack>
              <Title order={2}>Visibility</Title>
              <Text>
                Choose whether you want your report to be public or private.
                Public reports can be viewed by all users, while private reports
                are only visible to you.
              </Text>
              <Center>
                <SegmentedControl
                  w="50%"
                  data={[
                    {
                      label: (
                        <Center>
                          <Group gap={'xs'}>
                            <IconWorld />
                            Public
                          </Group>
                        </Center>
                      ),
                      value: 'true',
                    },
                    {
                      label: (
                        <Center>
                          <Group gap={'xs'}>
                            <IconLock />
                            Private
                          </Group>
                        </Center>
                      ),
                      value: 'false',
                    },
                  ]}
                  {...form.getInputProps('isPublic')}
                />
              </Center>
            </Stack>
          </Card>

          <Card>
            <Stack>
              <Title order={2}>Data Sources</Title>
              <Text>Add sources to fetch Reddit data from.</Text>
              <Box>
                <DataSourceTable
                  form={form}
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
            </Stack>
          </Card>

          <Card>
            <Stack>
              <Title order={2}>Sorting</Title>
              <Text>
                Choose how you want the data to be sorted by Reddit. This will
                be applied to all data sources. It exactly mirrors the sorting
                options available on Reddit.
              </Text>
              <Group>
                <Select
                  name={'sortBy'}
                  label="Sort by"
                  {...form.getInputProps('sortBy')}
                  placeholder="Select sorting type"
                  data={[
                    { value: 'hot', label: 'Hot' },
                    { value: 'new', label: 'New' },
                    { value: 'rising', label: 'Rising' },
                    { value: 'top', label: 'Top' },
                    { value: 'controversial', label: 'Controversial' },
                  ]}
                />
                {
                  // Only show time selection if sorting by top or controversial
                  form.getInputProps('sortBy').value !== 'top' &&
                  form.getInputProps('sortBy').value !==
                    'controversial' ? null : (
                    <Select
                      name="time"
                      label="Time"
                      placeholder="Select time"
                      disabled={
                        form.getInputProps('sortBy').value !== 'top' &&
                        form.getInputProps('sortBy').value !== 'controversial'
                      }
                      {...form.getInputProps('time')}
                      data={[
                        { value: 'day', label: 'Past day' },
                        { value: 'week', label: 'Past week' },
                        { value: 'month', label: 'Past month' },
                        { value: 'year', label: 'Past year' },
                        { value: 'all', label: 'All time' },
                      ]}
                    />
                  )
                }
              </Group>
            </Stack>
          </Card>

          <Card>
            <Stack>
              <Title order={2}>Fetch Size</Title>
              <Text>
                Choose how many API requests should be used to fetch data for
                your report. Higher values result in longer generation time.
              </Text>
              <Group>
                <Select
                  name="size"
                  label="Size"
                  placeholder="Select size"
                  data={[
                    { value: '30', label: 'Small (30)' },
                    { value: '70', label: 'Medium (70)' },
                    { value: '100', label: 'Large (100)' },
                    { value: 'custom', label: 'Custom' },
                  ]}
                  onChange={handleSelectChange}
                  value={form.values.size}
                />
                {selectValue === 'custom' && (
                  <NumberInput
                    label="Custom Size"
                    min={0}
                    max={500}
                    onChange={(value) =>
                      form.setFieldValue('size', value.toString())
                    }
                  />
                )}
              </Group>
            </Stack>
          </Card>

          <Card>
            <Stack>
              <Title order={2}>Analyses</Title>
              <Text>
                Choose which analyses you want to perform on the fetched data.
              </Text>
              {/*TODO Wrap into two columns*/}
              <Stack>
                <Checkbox
                  value={form.values.analyses.language}
                  label="Language"
                  {...form.getInputProps('analyses.language', {
                    type: 'checkbox',
                  })}
                />
                <Checkbox
                  value={form.values.analyses.sentiment}
                  label="Sentiment"
                  {...form.getInputProps('analyses.sentiment', {
                    type: 'checkbox',
                  })}
                />
                <Checkbox
                  value={form.values.analyses.sarcasm}
                  label="Sarcasm"
                  {...form.getInputProps('analyses.sarcasm', {
                    type: 'checkbox',
                  })}
                />
                <Checkbox
                  value={form.values.analyses.spam}
                  label="Spam"
                  {...form.getInputProps('analyses.spam', { type: 'checkbox' })}
                />
                <Checkbox
                  value={form.values.analyses.politics}
                  label="Politics"
                  {...form.getInputProps('analyses.politics', {
                    type: 'checkbox',
                  })}
                />
                <Checkbox
                  value={form.values.analyses.hateSpeech}
                  label="Hate Speech"
                  {...form.getInputProps('analyses.hateSpeech', {
                    type: 'checkbox',
                  })}
                />
                <Checkbox
                  value={form.values.analyses.clickbait}
                  label="Clickbait"
                  {...form.getInputProps('analyses.clickbait', {
                    type: 'checkbox',
                  })}
                />
                <Checkbox
                  value={form.values.analyses.trolling}
                  label="Trolling"
                  {...form.getInputProps('analyses.trolling', {
                    type: 'checkbox',
                  })}
                />
              </Stack>
            </Stack>
          </Card>

          <Group justify="flex-end">
            <Button
              type="submit"
              onClick={() => {
                console.log(transformJson(form.values as ReportFormValues));
              }}
            >
              Create Report
            </Button>
          </Group>
        </Stack>
      </form>
    </Stack>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <Report />
    </ErrorBoundary>
  );
}
