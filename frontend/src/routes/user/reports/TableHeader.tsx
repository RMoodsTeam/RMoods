import React from 'react';
import { Box, Title, Checkbox, Group } from '@mantine/core';

interface TableHeaderProps {
  checkAll: boolean;
  onCheckAll: (event: React.ChangeEvent<HTMLInputElement>) => void;
  onSort: (field: string) => void;
  sortField: string;
  sortOrder: 'asc' | 'desc';
}

const TableHeader: React.FC<TableHeaderProps> = ({ checkAll, onCheckAll, onSort, sortField, sortOrder }) => {
  const renderSortIcon = (field: string) => {
    if (sortField === field) {
      return sortOrder === 'asc' ? '↑' : '↓';
    }
    return '';
  };

  return (
    <Group style={{ marginTop: '50px', marginBottom: '10px' }} align="center">
      <Box style={{ width: '5%', marginLeft: '1.25rem' }}>
        <Checkbox
          checked={checkAll}
          onChange={onCheckAll}
        />
      </Box>
      <Box style={{ width: '20%', textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('title')}>
        <Title order={2}>Title {renderSortIcon('title')}</Title>
      </Box>
      <Box style={{ width: '35%', textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('analyses')}>
        <Title order={2}>Analyses {renderSortIcon('analyses')}</Title>
      </Box>
      <Box style={{ width: '10%', textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('status')}>
        <Title order={2}>Status {renderSortIcon('status')}</Title>
      </Box>
      <Box style={{ width: '20%', textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('created_at')}>
        <Title order={2}>Creation Time {renderSortIcon('created_at')}</Title>
      </Box>
    </Group>
  );
};

export default TableHeader;