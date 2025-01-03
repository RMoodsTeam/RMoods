import React from 'react';
import { Card, Text, Checkbox, Group, Box } from '@mantine/core';
import { Report, ReportStatus, ReportAnalysesMap } from '../../../rmoods/types.ts';

interface ReportCardProps {
  report: Report;
  onCheck: (id: string, checked: boolean) => void;
  checked: boolean;
}

const ReportCard: React.FC<ReportCardProps> = ({ report, onCheck, checked }) => {
  const handleCheck = (event: React.ChangeEvent<HTMLInputElement>) => {
    onCheck(report.id, event.target.checked);
  };

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

  const renderAnalyses = (analyses: ReportAnalysesMap) => {
    return Object.entries(analyses).map(([kind, analysis]) => (
      <Text key={kind} size='xl'>
        {kind}: {JSON.stringify(analysis)}
      </Text>
    ));
  };

  return (
    <Card shadow="sm" padding="lg" radius='md' style={{ marginBottom: '10px', display: 'flex' }}>
      <Group align="center">
        <Box style={{ flex: 0.5 }}>
          <Checkbox onChange={handleCheck} checked={checked} />
        </Box>
        <Box style={{ flex: 2, textAlign: 'left' }}>
          <Text size='xl'>{report.title}</Text>
        </Box>
        <Box style={{ flex: 4, textAlign: 'left' }}>
          {renderAnalyses(report.analyses)}
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