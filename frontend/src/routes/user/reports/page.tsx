import {
  ActionIcon,
  Anchor,
  Box,
  Center,
  Flex,
  Group,
  Loader,
  Pagination,
  Popover,
  Stack,
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
  ReportQuery,
  ReportStatus,
  ReportStatusKind,
} from '../../../rmoods/types.ts';
import { Link, useNavigate } from 'react-router-dom';
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
import { getLocalUnixTimestamp } from '../../../utility/util.ts';

export type MyReportsPageReportQuery = Omit<
  ReportQuery,
  'userNamePattern' | 'includeMyReports'
>;

const UserReportsPage = () => {
  const [reports, setReports] = useState<Report[]>([]);
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(true);

  // Server side sorting params
  const [sortField, setSortField] = useState<keyof Report>('created_at');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  const [page, setPage] = useState<number>(1);
  const [prevPage, setPrevPage] = useState<number>(1);

  const [filters, setFilters] = useState<MyReportsPageReportQuery>({
    titlePattern: '',
    startDate: undefined,
    endDate: undefined,
    containedAnalysisKinds: [],
    perPage: 30,
    page: 1,
  });

  const navigate = useNavigate();
  const userInfo = useAtomValue(userInfoAtom);

  useEffect(() => {
    const fetchReports = async () => {
      try {
        console.log(filters);
        if (userInfo?.name) {
          const urlParams = new URLSearchParams(location.search);
          if (filters.perPage)
            urlParams.set('per_page', filters.perPage.toString());
          if (filters.titlePattern)
            urlParams.set('title', filters.titlePattern);
          if (filters.startDate)
            urlParams.set(
              'start_date',
              getLocalUnixTimestamp(filters.startDate).toString()
            );
          if (filters.endDate)
            urlParams.set(
              'end_date',
              getLocalUnixTimestamp(filters.endDate).toString()
            );
          urlParams.set('page', page.toString());

          filters.containedAnalysisKinds.forEach((kind: NlpAnalysisKind) =>
            urlParams.append('analyses', kind)
          );
          navigate({ search: urlParams.toString() });

          // hidden from the user: append username and mine to the query
          urlParams.set('username', userInfo.name);
          urlParams.set('mine', 'true');
          const queryResponse = await RMoodsClient.fetchUserReports(urlParams);
          // Remove the username and mine params from the query
          urlParams.delete('username');
          urlParams.delete('mine');

          console.log(queryResponse);
          setReports(queryResponse.reports);
          setTotalPages(queryResponse.totalPages);
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

  const handleFilterChange = (
    field: keyof MyReportsPageReportQuery,
    value: string
  ) => {
    setFilters((prev) => ({
      ...prev,
      [field]: field === 'perPage' && !value ? 10 : value,
    }));
    if (field !== 'page') {
      setPage(1);
    }
  };

  const clearFilters = () => {
    setFilters({
      ...filters,
      titlePattern: '',
      startDate: undefined,
      endDate: undefined,
      containedAnalysisKinds: [],
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
      <Stack>
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
                <Table.Td>
                  <Anchor component={Link} to={`/report/${report.id}`}>
                    {report.title}
                  </Anchor>
                </Table.Td>
                <Table.Td>{report.description}</Table.Td>
                <Table.Td>
                  {dayjs(report.created_at).format('YYYY-MM-DD HH:mm')}
                </Table.Td>
                <Table.Td>
                  <Center>{renderStatus(report.status)}</Center>
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>

        <Center>
          <Pagination total={totalPages} value={page} onChange={setPage} />
        </Center>
      </Stack>
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
