import { Box, Title, Stack, Group, Button, Loader } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';
import { RMoodsClient } from '../../../rmoods/client/RMoodsClient';
import { useState, useEffect } from 'react';
import ReportCard from './ReportCard';
import { Report, NlpAnalysisKind } from '../../../rmoods/types.ts';
import { useNavigate } from 'react-router-dom';
import FilterNavbar from './FilterNavbar.tsx';
import { useAtomValue } from 'jotai';
import { userInfoAtom } from '../../../atoms.ts';
import TableHeader from './TableHeader.tsx';
import { open } from 'cypress';


const UserReportsPage = () => {
  const [reports, setReports] = useState<Report[]>([]);
  const [loading, setLoading] = useState(true);
  const [checkedReports, setCheckedReports] = useState<{ [key: string]: boolean }>({});
  const [checkAll, setCheckAll] = useState(false);
  const [filters, setFilters] = useState<{
    title: string;
    startDate: Date | null;
    endDate: Date | null;
    nlpKinds: NlpAnalysisKind[];
    reportsPerPage: number;
  }>({
    title: '',
    startDate: null,
    endDate: null,
    nlpKinds: [],
    reportsPerPage: 10,
  });
  const [sortField, setSortField] = useState<string>('created_at');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');
  const [page, setPage] = useState<number>(1);
  const [prevPage, setPrevPage] = useState<number>(1);

  const navigate = useNavigate();
  const userInfo = useAtomValue(userInfoAtom);

  useEffect(() => {
    const fetchReports = async () => {
      try {
        if (userInfo?.name) {
          const params = new URLSearchParams(location.search);
          params.set('username', userInfo.name);
          if (filters.reportsPerPage) params.set('per_page', filters.reportsPerPage.toString());
          if (filters.title) params.set('title', filters.title);
          if (filters.startDate) params.set('start_date', filters.startDate.toISOString());
          if (filters.endDate) params.set('end_date', filters.endDate.toISOString());
          params.set('page', page.toString());
          const data = await RMoodsClient.fetchUserReports(params.toString());
          setReports(data);
        } else {
          throw new Error('User name is undefined');
        }
      } catch (err) {
        console.error('Failed to fetch reports:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchReports();
  }, [location.search, userInfo]);

  useEffect(() => {
    const applyFilters = async () => {
      const params = new URLSearchParams();
      if (filters.title) params.set('title', filters.title);
      if (filters.startDate) params.set('start_date', filters.startDate.toISOString());
      if (filters.endDate) params.set('end_date', filters.endDate.toISOString());
      if (filters.reportsPerPage) params.set('per_page', filters.reportsPerPage.toString());
      params.set('page', page.toString());
      filters.nlpKinds.forEach(kind => params.append('analyses', kind));
      navigate({ search: params.toString() });

      try {
        if (userInfo?.name) {
          params.set('username', userInfo.name);
          const data = await RMoodsClient.fetchUserReports(params.toString());
          if (data.length === 0 && page > 1) {
            setPage(prevPage);
          } else {
            setPrevPage(page);
            setReports(data);
          }
        } else {
          throw new Error('User name is undefined');
        }
      } catch (err) {
        console.error('Failed to fetch reports:', err);
      }
    };
    const timeoutId = setTimeout(applyFilters, 500);
    return () => clearTimeout(timeoutId);
  }, [filters, page, userInfo, navigate]);

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

  const handleFilterChange = (field: string, value: any) => {
    setFilters((prev) => ({ ...prev, [field]: value }));
    if (field !== 'page') {
      setPage(1);
    }
  };

  const clearFilters = () => {
    setFilters({
      title: '',
      startDate: null,
      endDate: null,
      nlpKinds: [],
      reportsPerPage: 10,
    });
    navigate({ search: '' });
  }

  const handleSort = (field: string) => {
    if (sortField === field) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortOrder('asc');
    }
  };

  const sortedReports = [...reports].sort((a, b) => {
    const fieldA = a[sortField as keyof Report];
    const fieldB = b[sortField as keyof Report];

    if (fieldA < fieldB) {
      return sortOrder === 'asc' ? -1 : 1;
    }
    if (fieldA > fieldB) {
      return sortOrder === 'asc' ? 1 : -1;
    }
    return 0;
  });

  if (loading) {
    return <Loader />;
  }

  return (
    <Box style={{ width: '100%' }}>
      <Group justify="center" style={{ marginBottom: '30px' }}>
        <Title order={1}>My Reports</Title>
      </Group>

      <FilterNavbar
        filters={filters}
        onFilterChange={handleFilterChange}
        onClearFilters={clearFilters}
      />

      <TableHeader
        checkAll={checkAll}
        onCheckAll={handleCheckAll}
        onSort={handleSort}
        sortField={sortField}
        sortOrder={sortOrder}
      />

      <Stack style={{ marginTop: '20px' }}>
        {sortedReports.map((report) => (
          <ReportCard
            key={report.id}
            report={report}
            onCheck={handleCheck}
            checked={checkedReports[report.id] || false} />
        ))}
      </Stack>

      <Group justify="center" style={{ marginTop: '20px' }}>
        <Button onClick={() => setPage((prev) => Math.max(prev - 1, 1))} disabled={page === 1}>
          Previous
        </Button>
        <Button onClick={() => setPage((prev) => prev + 1)}>
          Next
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
