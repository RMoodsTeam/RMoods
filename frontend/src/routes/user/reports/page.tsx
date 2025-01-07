import {
  ActionIcon,
  Box,
  Button,
  Center,
  Flex,
  Group,
  Loader,
  Popover,
  Table,
  Title,
} from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';
import { RMoodsClient } from '../../../rmoods/client/RMoodsClient';
import { useEffect, useState } from 'react';
import {
  NlpAnalysisKind,
  Report,
  ReportStatus,
  ReportStatusKind,
} from '../../../rmoods/types.ts';
import { useNavigate } from 'react-router-dom';
import FilterNavbar from './FilterNavbar.tsx';
import { useAtomValue } from 'jotai';
import { userInfoAtom } from '../../../atoms.ts';
import {
  IconArrowNarrowDown,
  IconArrowNarrowUp,
  IconCheck,
  IconX,
} from '@tabler/icons-react';
import dayjs from 'dayjs';

const UserReportsPage = () => {
  const [reports, setReports] = useState<Report[]>([]);
  const [loading, setLoading] = useState(true);
  const [sortField, setSortField] = useState<keyof Report>('created_at');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');
  const [page, setPage] = useState<number>(1);
  const [prevPage, setPrevPage] = useState<number>(1);
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
    reportsPerPage: 30,
  });

  const navigate = useNavigate();
  const userInfo = useAtomValue(userInfoAtom);

  useEffect(() => {
    const fetchReports = async () => {
      try {
        if (userInfo?.name) {
          const urlParams = new URLSearchParams(location.search);
          urlParams.set('username', userInfo.name);
          urlParams.set('mine', 'true');
          if (filters.reportsPerPage)
            urlParams.set('per_page', filters.reportsPerPage.toString());
          if (filters.title) urlParams.set('title', filters.title);
          if (filters.startDate)
            urlParams.set('start_date', filters.startDate.toISOString());
          if (filters.endDate)
            urlParams.set('end_date', filters.endDate.toISOString());
          urlParams.set('page', page.toString());

          filters.nlpKinds.forEach((kind) =>
            urlParams.append('analyses', kind)
          );
          navigate({ search: urlParams.toString() });

          const queryResponse = await RMoodsClient.fetchUserReports(urlParams);
          console.log(queryResponse);
          if (queryResponse.reports.length === 0 && page > 1) {
            setPage(prevPage);
          } else {
            setPrevPage(page);
            setReports(queryResponse.reports);
          }
          setReports(queryResponse.reports);
        } else {
          throw new Error('User name is undefined');
        }
      } catch (err) {
        console.error('Failed to fetch reports:', err);
      } finally {
        setLoading(false);
      }
    };

    const timeoutId = setTimeout(fetchReports, 500);
    return () => clearTimeout(timeoutId);
  }, [filters, page, navigate, location.search, userInfo]);

  const handleFilterChange = (field: string, value: string) => {
    setFilters((prev) => ({
      ...prev,
      [field]: field === 'reportsPerPage' && !value ? 10 : value,
    }));
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
  };

  const handleSort = (field: keyof Report) => {
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

  const renderSortIcon = (field: keyof Report) => {
    if (sortField === field) {
      return sortOrder === 'asc' ? (
        <IconArrowNarrowUp size={20} />
      ) : (
        <IconArrowNarrowDown size={20} />
      );
    }
    return <div style={{ width: 20 }}></div>;
  };

  const renderStatus = (reportStatus: ReportStatus) => {
    switch (reportStatus.status) {
      case ReportStatusKind.Success:
        return (
          <ActionIcon radius={20} color="green" variant="filled">
            <IconCheck />
          </ActionIcon>
        );
      case ReportStatusKind.InProgress:
        return (
          <ActionIcon radius={20} color="yellow" variant="filled">
            <Loader />
          </ActionIcon>
        );
      case ReportStatusKind.Error:
        return (
          <Popover position={'top'}>
            <Popover.Target>
              <ActionIcon radius={20} color="red" variant="filled">
                <IconX />
              </ActionIcon>
            </Popover.Target>
            <Popover.Dropdown>{reportStatus.message}</Popover.Dropdown>
          </Popover>
        );
      default:
        throw new Error('Invalid status');
    }
  };

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

      <Table stickyHeader withColumnBorders>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>
              <Flex
                onClick={() => handleSort('title')}
                justify={'space-between'}
              >
                Title
                {renderSortIcon('title')}
              </Flex>
            </Table.Th>
            <Table.Th>
              <Flex
                onClick={() => handleSort('description')}
                justify={'space-between'}
              >
                Description
                {renderSortIcon('description')}
              </Flex>
            </Table.Th>
            <Table.Th>
              <Flex
                onClick={() => handleSort('created_at')}
                justify={'space-between'}
              >
                Created at
                {renderSortIcon('created_at')}
              </Flex>
            </Table.Th>
            <Table.Th>Status</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {sortedReports.map((report) => (
            <Table.Tr key={report.id}>
              <Table.Td>{report.title}</Table.Td>
              <Table.Td>{report.description}</Table.Td>
              <Table.Td>
                {dayjs(new Date(report.created_at)).format(
                  'YYYY-MM-DD HH:mm:ss'
                )}
              </Table.Td>
              <Table.Td>
                <Center>{renderStatus(report.status)}</Center>
              </Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>

      <Group justify="center" style={{ marginTop: '20px' }}>
        <Button
          onClick={() => setPage((prev) => Math.max(prev - 1, 1))}
          disabled={page === 1}
        >
          Previous
        </Button>
        <Button onClick={() => setPage((prev) => prev + 1)}>Next</Button>
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
