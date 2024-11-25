import { Button, Center, NumberInput, Table, TextInput } from '@mantine/core';
import { TbPlus } from 'react-icons/tb';
import { DataSource } from '../../rmoods/client/types.ts';
import { useState } from 'react';

const InputRow = ({ setRows }: any) => {
  const [inputRow, setInputRow] = useState<DataSource>({
    name: '',
    postId: '',
    share: 0,
  });

  const makeInputChangeHandler = (field: keyof DataSource) => (event: any) => {
    setInputRow((prev) => ({
      ...prev,
      [field]: field === 'share' ? event : event.target?.value,
    }));
  };
  const handleAddRow = () => {
    setRows((prevRows: DataSource[]) => [...prevRows, inputRow]);
    setInputRow({ name: '', postId: '', share: 0 });
  };

  return (
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
          <Button variant="transparent" onClick={() => handleAddRow()}>
            <TbPlus color={'white'} size={24} />
          </Button>
        </Center>
      </Table.Th>
    </Table.Tr>
  );
};

export default InputRow;
