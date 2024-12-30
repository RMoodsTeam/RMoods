import { Box, Title, Stack, Group, Button, Checkbox, TextInput, Grid } from '@mantine/core';
import { DatePicker } from '@mantine/dates';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';
import { RMoodsClient } from '../../../rmoods/client/RMoodsClient';
import { useState, useEffect } from 'react';
import ReportCard from './ReportCard';
import { Report } from '../../../rmoods/types.ts';
import { useLocation, useNavigate } from 'react-router-dom';

interface ReportQuery {
  userNamePattern?: string;
  containedAnalysisKinds: NlpAnalysisKind[];
  startDate?: string;
  endDate?: string;
  titlePattern?: string;
  includeMyReports: boolean;
  pagination: DbPagination;
}

interface DbPagination {
  page: number;
  perPage: number;
}

enum NlpAnalysisKind {
  Clickbait = 'Clickbait',
  HateSpeech = 'HateSpeech',
  Keywords = 'Keywords',
  Language = 'Language',
  Politics = 'Politics',
  Sarcasm = 'Sarcasm',
  Sentiment = 'Sentiment',
  Spam = 'Spam',
}

const UserReportsPage = () => {
  const [reports, setReports] = useState<Report[]>([]);
  const [loading, setLoading] = useState(true);
  const [checkedReports, setCheckedReports] = useState<{ [key: string]: boolean }>({});
  const [checkAll, setCheckAll] = useState(false);
  const [filters, setFilters] = useState({
    title: '',
    startDate: null,
    endDate: null,
  });

  const location = useLocation();
  const navigate = useNavigate();

  useEffect(() => {
    const fetchReports = async () => {
      try {
        const userId = '';
        const data = await RMoodsClient.fetchUserReports(userId);
        console.log('Fetched reports:', data);
        setReports(data);
      } catch (err) {
        console.error('Failed to fetch reports:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchReports();
  }, []);

  const handleCheck = (id: string, checked: boolean) => {
    setCheckedReports((prev) => ({ ...prev, [id]: checked }));
  };

  const handleCheckAll = (event: React.ChangeEvent<HTMLInputElement>) => {
    const checked = event.target.checked;
    setCheckAll(checked);
    const newCheckedReports = reports.reduce((acc, report) => {
      acc[report.id] = checked;
      return acc;
    }, {} as { [key: string]: boolean });
    setCheckedReports(newCheckedReports);
  };

  const deleteCheckedReports = () => {
    const checkedIds = Object.keys(checkedReports).filter((id) => checkedReports[id]);
    console.log('Checked report IDs:', checkedIds);
  };

  const handleFilterChange = (field: string, value: any) => {
    setFilters((prev) => ({ ...prev, [field]: value }));
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <Box style={{ width: '100%' }}>
      <Group justify="center" style={{ marginBottom: '20px' }}>
        <Title order={1}>My Reports</Title>
      </Group>

      <Grid style={{ marginBottom: '20px' }}>
        <Grid.Col span={4}>
          <TextInput
            placeholder="Title"
            value={filters.title}
          />
        </Grid.Col>
        <Grid.Col span={3}>
          <DatePicker
            allowDeselect
            value={filters.endDate}
            onChange={(date) => handleFilterChange('endDate', date)}
            styles={{
              calendarHeaderControl: {
                fontSize: '1rem', 
                width: '2rem', 
                height: '2rem', 
              },
            }}
            size='xs'
          />
        </Grid.Col>
        <Grid.Col span={2}>
          <Button fullWidth>Apply Filters</Button>
        </Grid.Col>
      </Grid>

      <Group style={{ marginTop: '20px', marginBottom: '10px' }}>
        <Checkbox
          style={{ width: '6%', marginLeft: '1.25rem' }}
          checked={checkAll}
          onChange={handleCheckAll}
        />
        <Title order={2} fw={700} style={{ width: '23%' }}>Title</Title>
        <Title order={2} style={{ width: '31%' }}>Description</Title>
        <Title order={2} style={{ width: '25%' }}>Creation Time</Title>
      </Group>

      <Stack style={{ marginTop: '20px' }}>
        {reports.map((report) => (
          <ReportCard
            key={report.id}
            report={report}
            onCheck={handleCheck}
            checked={checkedReports[report.id] || false} />
        ))}
      </Stack>

      <Group justify="right" style={{ marginTop: '20px' }}>
        <Button onClick={deleteCheckedReports}>
          Delete Reports
        </Button>
      </Group>
    </Box>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <UserReportsPage />
    </ErrorBoundary>
  );
}
