import React from 'react';
import { Box, Text } from '@mantine/core';

/**
 * Props for the StatisticItem component.
 */
interface StatisticItemProps {
  label: string;
  value: string | number;
}

/**
 * StatisticItem component that displays a statistic with a label and value.
 * @param {StatisticItemProps} props - The props for the StatisticItem component.
 * @param {string} props.label - The label for the statistic.
 * @param {string | number} props.value - The value of the statistic.
 * @returns {JSX.Element} The StatisticItem component.
 */
const StatisticItem: React.FC<StatisticItemProps> = ({ label, value }) => {
  return (
    <Box style={{ width: '100%', marginBottom: '10px', height: '100px', border: '1px solid #ccc', borderRadius: '12px', padding: '20px' }}>
      <Text w={300}>{label}:</Text>
      <Text>{value}</Text>
    </Box>
  );
};

export default StatisticItem;