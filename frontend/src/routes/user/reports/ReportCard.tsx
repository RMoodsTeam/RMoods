import React from 'react';
import { Card, Text, Group, Box } from '@mantine/core';
import { Report, ReportStatus } from '../../../rmoods/types.ts';

interface ReportCardProps {
  report: Report;
}

const ReportCard: React.FC<ReportCardProps> = ({ report }) => {

  const renderStatus = (status: ReportStatus) => {
    if (typeof status === 'object' && 'Error' in status) {
      return <Text c="red" size='xl'>Error</Text>;
    }

    switch (status) {
      case ReportStatus.Success:
        return <Text c="green" size='xl'>Success</Text>;
      case ReportStatus.InProgress:
        return <Text c="blue" size='xl'>In Progress</Text>;
      default:
        return <Text c="gray" size='xl'>Unknown</Text>;
    }
  };

  return (
    <Card shadow="sm" padding="lg" radius='md' style={{ marginBottom: '10px', display: 'flex' }}>
      <Group align="center">
        <Box style={{ flex: 2, textAlign: 'left' }}>
          <Text size='xl'>{report.title}</Text>
        </Box>
        <Box style={{ flex: 4, textAlign: 'left' }}>
          <Text size='xl'>{report.description}</Text>
        </Box>
        <Box style={{ flex: 1, textAlign: 'left' }}>
          {renderStatus(report.status)}
        </Box>
        <Box style={{ flex: 2, textAlign: 'left' }}>
          <Text size='xl'>{new Date(report.created_at).toLocaleString()}</Text>
        </Box>
      </Group>
    </Card>
  );
};

export default ReportCard;