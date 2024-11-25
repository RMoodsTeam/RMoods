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
import { useState } from 'react';
import { TbPlus, TbTrash } from 'react-icons/tb';
import { DataSource } from '../../rmoods/client/types.ts';

const Report = () => {
  const form = useForm({
    initialValues: {
      name: '',
      resourceType: 'subredditPosts',
      isPublic: 'true',
      size: 'small',
      customSize: undefined,
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
  });
  const [rows, setRows] = useState<DataSource[]>([]); // Array to store all rows

  const makeInputChangeHandler = (field: keyof DataSource) => (event: any) => {
    setInputRow((prev) => ({
      ...prev,
      [field]: field === 'share' ? event : event.target?.value || event,
    }));
  };

  const [inputRow, setInputRow] = useState<DataSource>({
    name: '',
    postId: '',
    share: 0,
  });

  const handleAddRow = () => {
    setRows((prevRows) => [...prevRows, inputRow]);
    setInputRow({ name: '', postId: '', share: 0 });
  };

  const makeRowEditHandler =
    (index: number, property: keyof DataSource) => (event: any) => {
      setRows(
        rows.map((row, i) =>
          i === index
            ? {
                ...row,
                [property]:
                  property === 'share' ? event : event.target?.value || event,
              }
            : row
        )
      );
    };

  const deleteRow = (index: number) => {
    setRows((_) => rows.filter((_, i) => i !== index));
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
              <Table.Tr>
                <Table.Th>
                  <Center>
                    <TextInput
                      placeholder="eg. r/Polska"
                      onChange={makeInputChangeHandler('name')}
                      value={inputRow.name}
                    />
                  </Center>
                </Table.Th>
                <Table.Th>
                  <Center>
                    <TextInput
                      placeholder={'eg. 1gyonvx'}
                      onChange={makeInputChangeHandler('postId')}
                      value={inputRow.postId}
                    />
                  </Center>
                </Table.Th>
                <Table.Th>
                  <Center>
                    <NumberInput
                      placeholder="eg. 3"
                      onChange={makeInputChangeHandler('share')}
                      value={inputRow.share}
                    />
                  </Center>
                </Table.Th>
                <Table.Th>
                  <Center>
                    <Button
                      variant="transparent"
                      onClick={() => handleAddRow()}
                    >
                      <TbPlus color={'white'} size={24} />
                    </Button>
                  </Center>
                </Table.Th>
              </Table.Tr>
            </Table.Thead>

            <Table.Tbody>
              {rows.map((row, index) => (
                <Table.Tr key={index}>
                  <Table.Td>
                    <TextInput
                      onChange={makeRowEditHandler(index, 'name')}
                      variant="unstyled"
                      defaultValue={row.name}
                    />
                  </Table.Td>
                  <Table.Td>
                    <TextInput
                      onChange={makeRowEditHandler(index, 'postId')}
                      variant="unstyled"
                      defaultValue={row.postId}
                    />
                  </Table.Td>
                  <Table.Td>
                    <NumberInput
                      onChange={makeRowEditHandler(index, 'share')}
                      variant="unstyled"
                      defaultValue={row.share}
                    />
                  </Table.Td>
                  <Table.Td>
                    <Center>
                      <Button
                        color={'white'}
                        variant="transparent"
                        onClick={() => deleteRow(index)}
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
          <Button type="submit">Create Report</Button>
        </Group>
      </form>
    </Box>
  );
};

export default Report;
