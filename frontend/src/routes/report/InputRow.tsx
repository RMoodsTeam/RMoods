import { Button, Center, NumberInput, Table, TextInput } from '@mantine/core';
import { TbPlus } from 'react-icons/tb';
import { DataSource } from '../../rmoods/client/types.ts';
import { useState } from 'react';
import { RowWrapper } from './types.ts';

let ID = 1;

function generateId() {
  return ID++;
}

interface InputRowProps {
  setRows: any;
  form: any;
}

/**
 * Component that renders an input row for adding new data sources.
 *
 * @param {Object} props - The properties object.
 * @param {Function} props.setRows - Function to update the rows.
 * @param {any} props.form - The form object containing the values.
 * @returns {JSX.Element} - The rendered input row component.
 */
const InputRow = ({ setRows, form }: InputRowProps) => {
  const [inputRow, setInputRow] = useState<RowWrapper>({
    dataSource: {
      name: '',
      postId: '',
      share: 0,
    },
    id: 0,
  });

  /**
   * Creates a change handler for the input fields in the data source.
   *
   * @param {keyof DataSource} field - The field of the data source to be updated.
   * @returns {Function} - A function that handles the change event for the specified field.
   */
  const makeInputChangeHandler = (field: keyof DataSource) => (event: any) => {
    setInputRow((prev) => ({
      ...prev,
      dataSource: {
        ...prev.dataSource,
        [field]: field === 'share' ? event : event.target?.value,
      },
    }));
  };

  /**
   * Adds the current input row to the list of rows and resets the input row.
   *
   * @returns {void}
   */
  const handleAddRow = () => {
    setRows((prevRows: RowWrapper[]) => [...prevRows, inputRow]);
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
          />
        </Center>
      </Table.Th>
      {form.values.resourceType == 'postComments' && (
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
      <Table.Th>
        <Center>
          <NumberInput
            placeholder="eg. 3"
            onChange={makeInputChangeHandler('share')}
            value={inputRow.dataSource.share}
          />
        </Center>
      </Table.Th>
      <Table.Th>
        <Center>
          <Button variant="transparent" onClick={() => handleAddRow()}>
            <TbPlus color={'white'} size={24} />
          </Button>
        </Center>
      </Table.Th>
    </Table.Tr>
  );
};

export default InputRow;
