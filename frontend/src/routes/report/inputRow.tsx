import { Button, Center, NumberInput, Table, TextInput } from '@mantine/core';
import { TbPlus } from 'react-icons/tb';
import { DataSource, DataSourceSchema } from '../../rmoods/client/types.ts';
import { useState } from 'react';
import { z } from 'zod';

export const RowWrapperSchema = z.object({
  dataSource: DataSourceSchema,
  id: z.number(),
});

let ID = 1;

function generateId() {
  return ID++;
}

export type RowWrapper = z.infer<typeof RowWrapperSchema>;

const InputRow = ({ setRows }: any) => {
  const [inputRow, setInputRow] = useState<RowWrapper>({
    dataSource: {
      name: '',
      postId: '',
      share: 0,
    },
    id: 0,
  });

  const makeInputChangeHandler = (field: keyof DataSource) => (event: any) => {
    setInputRow((prev) => ({
      ...prev,
      dataSource: {
        ...prev.dataSource,
        [field]: field === 'share' ? event : event.target?.value,
      },
    }));
  };

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
            placeholder="eg. r/Polska"
            onChange={makeInputChangeHandler('name')}
            value={inputRow.dataSource.name}
          />
        </Center>
      </Table.Th>
      <Table.Th>
        <Center>
          <TextInput
            placeholder={'eg. 1gyonvx'}
            onChange={makeInputChangeHandler('postId')}
            value={inputRow.dataSource.postId}
          />
        </Center>
      </Table.Th>
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
