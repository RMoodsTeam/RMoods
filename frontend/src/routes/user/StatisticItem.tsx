import React from 'react';
import { Box, Text, Divider } from '@mantine/core';

interface StatisticItemProps {
  label: string;
  value: string | number;
}

const StatisticItem: React.FC<StatisticItemProps> = ({ label, value }) => {
  return (
    <Box style={{ width: '100%', marginBottom: '10px', height: '100px', border: '1px solid #ccc', borderRadius: '12px', padding: '20px' }}>
      <Text w={300}>{label}:</Text>
      <Text>{value}</Text>
    </Box>
  );
};

export default StatisticItem;