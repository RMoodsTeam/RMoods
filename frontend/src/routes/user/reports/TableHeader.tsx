import React from 'react';
import { Box, Title, Group } from '@mantine/core';

interface TableHeaderProps {
  onSort: (field: string) => void;
  sortField: string;
  sortOrder: 'asc' | 'desc';
}

const TableHeader: React.FC<TableHeaderProps> = ({ onSort, sortField, sortOrder }) => {
  const renderSortIcon = (field: string) => {
    if (sortField === field) {
      return sortOrder === 'asc' ? '↑' : '↓';
    }
    return '';
  };

  return (
    <Group style={{ marginTop: '50px', marginBottom: '10px' }} align="center">
      <Box style={{ flex: 2, textAlign: 'left', cursor: 'pointer', marginLeft: '1.25rem' }} onClick={() => onSort('title')}>
        <Title order={2}>Title {renderSortIcon('title')}</Title>
      </Box>
      <Box style={{ flex: 4, textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('description')}>
        <Title order={2}>Description {renderSortIcon('description')}</Title>
      </Box>
      <Box style={{ flex: 1, textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('status')}>
        <Title order={2}>Status {renderSortIcon('status')}</Title>
      </Box>
      <Box style={{ flex: 2, textAlign: 'left', cursor: 'pointer' }} onClick={() => onSort('created_at')}>
        <Title order={2}>Creation Time {renderSortIcon('created_at')}</Title>
      </Box>
    </Group>
  );
};

export default TableHeader;