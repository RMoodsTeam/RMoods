import { Button, Center, NumberInput, Table, TextInput } from '@mantine/core';
import { TbPlus } from 'react-icons/tb';
import { DataSource } from '../../rmoods/client/types.ts';
import { useState } from 'react';
import { ReportFormValues, RowWrapper } from './types.ts';
import { UseFormReturnType } from '@mantine/form';

let ID = 1;

function generateId() {
  return ID++;
}

interface InputRowProps {
  setRows: any;
  form: UseFormReturnType<ReportFormValues>;
  manualShares?: boolean;
}

/**
 * Component that renders an input row for adding new data sources.
 *
 * @param {Object} props - The properties object.
 * @param {Function} props.setRows - Function to update the rows.
 * @param {any} props.form - The form object containing the values.
 * @returns {JSX.Element} - The rendered input row component.
 */
const InputRow = ({ setRows, form, manualShares }: InputRowProps) => {
  const [inputRow, setInputRow] = useState<RowWrapper>({
    dataSource: {
      name: '',
      postId: '',
      share: 0,
    },
    id: 0,
  });

  // Create a function that will mutate the given data
  const makeInputChangeHandler = (field: keyof DataSource) => (event: any) => {
    setInputRow((prev) => ({
      ...prev,
      dataSource: {
        ...prev.dataSource,
        [field]: field === 'share' ? event : event.target?.value,
      },
    }));
  };

  const handleAddRow = (): void => {
    // If manual shares are enabled, add the row as is,
    // otherwise, distribute the shares evenly across all rows
    setRows((prevRows) => {
      if (manualShares) {
        return [...prevRows, inputRow];
      }

      const totalRows = prevRows.length + 1;
      const baseShare = Math.floor(100 / totalRows);
      const remainder = 100 % totalRows;

      return [...prevRows, inputRow].map((row, index) => ({
        ...row,
        dataSource: {
          ...row.dataSource,
          share: index < remainder ? baseShare + 1 : baseShare,
        },
      }));
    });

    // Reset the input row
    setInputRow({
      dataSource: { name: '', postId: '', share: 0 },
      id: generateId(),
    });
  };

  return (
    <Table.Tr>
      <Table.Th>
        <Center>
          <TextInput
            placeholder="eg. Polska"
            onChange={makeInputChangeHandler('name')}
            value={inputRow.dataSource.name}
            onKeyUp={(e) => {
              if (e.key === 'Enter') {
                e.stopPropagation();
                e.preventDefault();
                handleAddRow(); // for accessibility reasons, only add a row on key up event
              }
            }}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                e.stopPropagation();
                e.preventDefault();
              }
            }}
          />
        </Center>
      </Table.Th>
      {form.values.resourceKind == 'postComments' && (
        <Table.Th>
          <Center>
            <TextInput
              placeholder={'eg. 1gyonvx'}
              onChange={makeInputChangeHandler('postId')}
              value={inputRow.dataSource.postId}
            />
          </Center>
        </Table.Th>
      )}
      {manualShares ? (
        <Table.Th>
          <Center>
            <NumberInput
              placeholder="eg. 3"
              onChange={makeInputChangeHandler('share')}
              value={inputRow.dataSource.share}
            />
          </Center>
        </Table.Th>
      ) : (
        <></>
      )}
      <Table.Th>
        <Center>
          <Button variant="transparent" onClick={() => handleAddRow()}>
            <TbPlus size={24} />
          </Button>
        </Center>
      </Table.Th>
    </Table.Tr>
  );
};

export default InputRow;
