import {
  Button,
  Center,
  Group,
  NumberInput,
  SegmentedControl,
  Stack,
  Switch,
  Table,
  TextInput,
} from '@mantine/core';
import InputRow from './InputRow.tsx';
import { TbTrash } from 'react-icons/tb';
import { DataSource } from './schema.ts';
import { MantineReportForm, RowWrapper } from './schema.ts';
import { useState } from 'react';
import { IconBrandReddit, IconMessage, IconUser } from '@tabler/icons-react';
import gradientSegmentControlClasses from './GradientSegmentedControl.module.scss';

interface TableProps {
  rows: RowWrapper[];
  setRows: CallableFunction;
  makeRowEditHandler: (
    index: number,
    property: keyof DataSource
  ) => (event: unknown) => void;
  deleteRow: (id: number) => void;
  form: MantineReportForm;
}

/**
 * Returns the column name based on the resource type in the form values.
 *
 * @param {MantineReportForm} form - The form object containing the values.
 * @returns {string} - The column name ('Post', 'Comment', or 'Name').
 */
const getResourceKindColumnName = (form: MantineReportForm): string => {
  if (form.values.resourceKind === 'userPosts') {
    return 'Username';
  } else {
    return 'Subreddit';
  }
};

/**
 * Component that renders a table for displaying and editing data sources.
 *
 * @param {Object} props - The properties object.
 * @param {RowWrapper[]} props.rows - The array of row data.
 * @param {Function} props.setRows - Function to update the rows.
 * @param {Function} props.makeRowEditHandler - Function to create an edit handler for a row.
 * @param {Function} props.deleteRow - Function to delete a row by its ID.
 * @param {any} props.form - The form object containing the values.
 * @returns {JSX.Element} - The rendered table component.
 */
export const DataSourceTable = ({
  rows,
  setRows,
  makeRowEditHandler,
  deleteRow,
  form,
}: TableProps) => {
  const [manualShares, setManualShares] = useState<boolean>(false);

  return (
    <Stack>
      <Switch
        checked={manualShares}
        label="Manually assign shares"
        onChange={(event: any) => {
          setManualShares(event.currentTarget.checked);
        }}
      />
      <Center>
        <Stack>
          <SegmentedControl
            radius="xl"
            size="sm"
            classNames={gradientSegmentControlClasses}
            data={[
              {
                label: (
                  <Group wrap={'nowrap'}>
                    <IconBrandReddit />
                    Subreddit Posts
                  </Group>
                ),
                value: 'subredditPosts',
              },
              {
                label: (
                  <Group wrap={'nowrap'}>
                    <IconUser />
                    User Posts
                  </Group>
                ),
                value: 'userPosts',
              },
              {
                label: (
                  <Group wrap={'nowrap'}>
                    <IconMessage />
                    Post Comments
                  </Group>
                ),
                value: 'postComments',
              },
            ]}
            {...form.getInputProps('resourceKind')}
            onClick={() => {
              setRows([]);
            }}
          />
        </Stack>
      </Center>

      <Center>
        <Table striped highlightOnHover w={'75%'}>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>
                <Center>{getResourceKindColumnName(form)}</Center>
              </Table.Th>
              {form.values.resourceKind == 'postComments' && (
                <Table.Th>
                  <Center>Post ID</Center>
                </Table.Th>
              )}
              {manualShares ? (
                <Table.Th>
                  <Center>Share</Center>
                </Table.Th>
              ) : (
                <></>
              )}
              <Table.Th></Table.Th>
            </Table.Tr>
            <InputRow
              setRows={setRows}
              manualShares={manualShares}
              form={form}
            />
          </Table.Thead>

          <Table.Tbody>
            {rows.map((row, index) => (
              <Table.Tr key={row.id}>
                <Table.Td>
                  <Center>
                    <TextInput
                      onChange={makeRowEditHandler(index, 'name')}
                      variant="unstyled"
                      defaultValue={row.dataSource.name}
                    />
                  </Center>
                </Table.Td>
                {form.values.resourceKind == 'postComments' && (
                  <Table.Td>
                    <Center>
                      <TextInput
                        onChange={makeRowEditHandler(index, 'postId')}
                        variant="unstyled"
                        defaultValue={row.dataSource.postId}
                      />
                    </Center>
                  </Table.Td>
                )}
                {manualShares ? (
                  <Table.Td>
                    <Center>
                      <NumberInput
                        onChange={makeRowEditHandler(index, 'share')}
                        variant="unstyled"
                        value={row.dataSource.share}
                      />
                    </Center>
                  </Table.Td>
                ) : (
                  <></>
                )}
                <Table.Td>
                  <Center>
                    <Button
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
      </Center>
    </Stack>
  );
};
