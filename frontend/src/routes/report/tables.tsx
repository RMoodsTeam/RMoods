import { Button, Center, NumberInput, Table, TextInput } from '@mantine/core';
import InputRow, { RowWrapper } from './inputRow.tsx';
import { TbTrash } from 'react-icons/tb';
import { DataSource } from '../../rmoods/client/types.ts';

interface tableProps {
  rows: RowWrapper[];
  setRows: any;
  makeRowEditHandler: (
    index: number,
    property: keyof DataSource
  ) => (event: any) => void;
  deleteRow: (id: number) => void;
  form: any;
}

const getResourceTypeColumnName = (form: any) => {
  if (form.values.resourceType === 'subredditPosts') {
    return 'Post';
  } else if (form.values.resourceType === 'postComments') {
    return 'Comment';
  } else {
    return 'Name';
  }
};

export const DataSourceTable = ({
  rows,
  setRows,
  makeRowEditHandler,
  deleteRow,
  form,
}: tableProps) => {
  return (
    <Table striped highlightOnHover>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>
            <Center>{getResourceTypeColumnName(form)}</Center>
          </Table.Th>
          {form.values.resourceType !== 'postComments' && (
            <Table.Th>
              <Center>Post ID</Center>
            </Table.Th>
          )}
          <Table.Th>
            <Center>Share</Center>
          </Table.Th>
          <Table.Th></Table.Th>
        </Table.Tr>
        <InputRow form={form} setRows={setRows} />
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
            {form.values.resourceType !== 'postComments' && (
              <Table.Td>
                <TextInput
                  onChange={makeRowEditHandler(index, 'postId')}
                  variant="unstyled"
                  defaultValue={row.dataSource.postId}
                />
              </Table.Td>
            )}
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
  );
};
