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
  Table,
  Center,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { useEffect, useState } from 'react';
import InputRow, { RowWrapperSchema } from './inputRow.tsx';
import { RowWrapper } from './inputRow.tsx';
import {
  AnalysisTypeSchema,
  DataSource,
  DataSourceSchema,
  FeedKindSchema,
} from '../../rmoods/client/types.ts';
import { zodResolver } from 'mantine-form-zod-resolver';
import { TbTrash } from 'react-icons/tb';
import { boolean, z } from 'zod';

const Report = () => {
  const form = useForm({
    initialValues: {
      name: '',
      resourceType: 'subredditPosts',
      isPublic: 'true',
      size: 'small',
      customSize: undefined,
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
    validate: zodResolver(
      z.object({
        name: z.string().min(1),
        resourceType: FeedKindSchema,
        isPublic: z.enum(['true', 'false']),
        size: z.enum(['small', 'medium', 'large', 'custom']),
        customSize: z.number().optional(),
        dataSource: z.array(DataSourceSchema),
        analyses: AnalysisTypeSchema,
      })
    ),
  });

  // form.setFieldError('name', 'Name is required');
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
        <Box style={{ border: '1px solid red' }}>
          <Table striped highlightOnHover>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>
                  <Center>Name</Center>
                </Table.Th>
                <Table.Th>
                  <Center>Post ID</Center>
                </Table.Th>
                <Table.Th>
                  <Center>Share</Center>
                </Table.Th>
                <Table.Th></Table.Th>
              </Table.Tr>
              <InputRow setRows={setRows} />
            </Table.Thead>
            <Table.Tbody>
              {rows.map((row, index) => (
                <Table.Tr key={row.id}>
                  <Table.Td>
                    <TextInput
                      onChange={makeRowEditHandler(index, 'name')}
                      variant="unstyled"
                      defaultValue={row.dataSource.name}
                    />
                  </Table.Td>
                  <Table.Td>
                    <TextInput
                      onChange={makeRowEditHandler(index, 'postId')}
                      variant="unstyled"
                      defaultValue={row.dataSource.postId}
                    />
                  </Table.Td>
                  <Table.Td>
                    <NumberInput
                      onChange={makeRowEditHandler(index, 'share')}
                      variant="unstyled"
                      defaultValue={row.dataSource.share}
                    />
                  </Table.Td>
                  <Table.Td>
                    <Center>
                      <Button
                        color={'white'}
                        variant="transparent"
                        onClick={() => {
                          deleteRow(row.id);
                        }}
                      >
                        <TbTrash size={24} />
                      </Button>
                    </Center>
                  </Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Box>

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
              console.log(form.errors);
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
