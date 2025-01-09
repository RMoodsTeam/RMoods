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
import { addDays } from 'date-fns';

export type MyReportsPageReportQuery = Omit<
  ReportQuery,
  'userNamePattern' | 'includeMyReports'
>;

/**
 * Converts a date to a Unix timestamp in seconds, adjusted for local timezone.
 */
export function shiftToUTC(date: Date): number {
  const newDate = new Date(date.getTime() + date.getTimezoneOffset());
  return Math.ceil(newDate.getTime() / 1000);
}

/*
 * Converts a date to a Unix timestamp in seconds, adjusted for local timezone.
 *
 * Because this is the start date, it should cover the entire day of the start date.
 * So we set the hours, minutes, seconds, and milliseconds to 0.
 *
 * It's then converted to a UTC timestamp in seconds.
 * So, if we're in GMT+1, and the date is 2025-01-04 00:02:00, the timestamp will be one for 2025-01-04 00:01:00.
 *
 * All of this is because the backend only accepts UTC timestamps, without any consideration for the user's timezone.
 *
 * Example:
 *
 * 1. User selects 2025-01-04 00:02:00 while being in GMT+1.
 * 2. We need to send two timestamps, that when interpreted as UTC, will cover the entire day of 2025-01-04 in GMT+1.
 * 3. So, we need to send 2025-01-03 23:00:00 and 2025-01-04 23:00:00, because when GMT+1 offset is applied to those timestamps, they will cover the entire day of 2025-01-04.
 * 4. We set the hours, minutes, seconds, and milliseconds to 0, because we want to cover the entire day.
 * 5. We convert the timestamps to UTC.
 */
function processStartDate(startDate: Date): number {
  const start = new Date(startDate);
  start.setHours(0, 0, 0, 0);
  return shiftToUTC(start);
}

/*
 See `processStartDate` for a detailed explanation.
 */
function processEndDate(endDate: Date): number {
  // Example: User selects 2025-01-04 00:00:00
  // We add 1 day, so it becomes 2025-01-05 00:00:00
  // We set the hours, minutes, seconds, and milliseconds to 0, so it becomes 2025-01-05 00:00:00
  // Thus, the entire day of 2025-01-04 is covered.
  const end = addDays(new Date(endDate), 1);
  end.setHours(0, 0, 0, 0);
  return shiftToUTC(end);
}

const UserReportsPage = () => {
  const [reports, setReports] = useState<Report[]>([]);
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(true);

  // Server side sorting params
  const [sortField, setSortField] = useState<keyof Report>('created_at');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  const [page, setPage] = useState<number>(1);

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
        const urlParams = new URLSearchParams(location.search);
        if (filters.perPage)
          urlParams.set('per_page', filters.perPage.toString());
        if (filters.titlePattern) urlParams.set('title', filters.titlePattern);
        if (filters.startDate) {
          const startDate = processStartDate(filters.startDate);
          urlParams.set('start_date', startDate.toString());
        }
        if (filters.endDate) {
          const endDate = processEndDate(filters.endDate);
          urlParams.set('end_date', endDate.toString());
        }
        filters.containedAnalysisKinds.forEach((kind: NlpAnalysisKind) =>
          urlParams.append('analyses', kind)
        );
        urlParams.set('page', page.toString());
        navigate({ search: urlParams.toString() });

        // hidden from the user: append username and mine to the query
        urlParams.set('username', userInfo!.name);
        urlParams.set('mine', 'true');
        const queryResponse = await RMoodsClient.fetchUserReports(urlParams);
        // Remove the username and mine params from the query
        urlParams.delete('username');
        urlParams.delete('mine');

        setReports(queryResponse.reports);
        setTotalPages(queryResponse.totalPages);
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
          <ActionIcon size={24} radius={20} color="green" variant="filled">
            <IconCheck />
          </ActionIcon>
        );
      case ReportStatusKind.InProgress:
        return <Loader size={24} color={'yellow'} />;
      case ReportStatusKind.Error:
        return (
          <Popover position={'top'}>
            <Popover.Target>
              <ActionIcon size={24} radius={20} color="red" variant="filled">
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
