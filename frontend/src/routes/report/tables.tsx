import { Button, Center, NumberInput, Table, TextInput } from '@mantine/core';
import InputRow from './InputRow.tsx';
import { TbTrash } from 'react-icons/tb';
import { DataSource } from '../../rmoods/client/types.ts';
import { ReportFormValues, RowWrapper } from './types.ts';
import { UseFormReturnType } from '@mantine/form';

interface TableProps {
  rows: RowWrapper[];
  setRows: any;
  makeRowEditHandler: (
    index: number,
    property: keyof DataSource
  ) => (event: any) => void;
  deleteRow: (id: number) => void;
  form: UseFormReturnType<ReportFormValues>;
}

/**
 * Returns the column name based on the resource type in the form values.
 *
 * @param {UseFormReturnType<ReportFormValues>} form - The form object containing the values.
 * @returns {string} - The column name ('Post', 'Comment', or 'Name').
 */
const getResourceKindColumnName = (
  form: UseFormReturnType<ReportFormValues>
): string => {
  if (form.values.resourceKind === 'subredditPosts') {
    return 'Subreddit';
  } else if (form.values.resourceKind === 'postComments') {
    return 'Comment';
  } else {
    return 'Username';
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
  return (
    <Table striped highlightOnHover>
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
            {form.values.resourceKind == 'postComments' && (
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
