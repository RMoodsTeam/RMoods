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
  Tooltip,
} from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../../PageFallback';
import { RMoodsClient } from '../../../rmoods/client/RMoodsClient';
import { useEffect, useState } from 'react';
import {
  Report,
  ReportQuery,
  ReportStatus,
  ReportStatusKind,
} from '../../../rmoods/types.ts';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import FilterNavbar from './FilterNavbar.tsx';
import {
  IconArrowNarrowDown,
  IconArrowNarrowUp,
  IconCheck,
  IconX,
} from '@tabler/icons-react';
import dayjs from 'dayjs';
import { addDays } from 'date-fns';
import { useQuery } from '@tanstack/react-query';
import { extractAnalysisKinds, truncateText } from '../../../utility/util.ts';

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

function useDebounce(value: string, delay: number) {
  const [debouncedValue, setDebouncedValue] = useState(value);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
}

const fetchReports = async (params: URLSearchParams) => {
  const queryResponse = await RMoodsClient.fetchUserReports(params);
  return queryResponse;
};

const BrowseReportsPage = () => {
  const [filters, setFilters] = useState<ReportQuery>({
    titlePattern: '',
    startDate: undefined,
    endDate: undefined,
    containedAnalysisKinds: [],
    perPage: 30,
    page: 1,
    userNamePattern: '',
    includeMyReports: true,
  });

  const [page, setPage] = useState<number>(1);
  const [sortField, setSortField] = useState<keyof Report>('created_at');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  const navigate = useNavigate();
  const location = useLocation();

  const debouncedTitlePattern = useDebounce(filters.titlePattern || '', 500);

  useEffect(() => {
    const queryParams = new URLSearchParams(location.search);
    if (filters.perPage) queryParams.set('per_page', filters.perPage.toString());
    if (debouncedTitlePattern) queryParams.set('title', debouncedTitlePattern)
    else queryParams.delete('title');
    if (filters.startDate) queryParams.set('start_date', processStartDate(filters.startDate).toString());
    if (filters.endDate) queryParams.set('end_date', processEndDate(filters.endDate).toString());

    queryParams.delete('analyses');
    if (filters.containedAnalysisKinds.length !== 0) {
      filters.containedAnalysisKinds.forEach((kind) => queryParams.append('analyses', kind))
    }

    if (filters.userNamePattern) queryParams.set('username', filters.userNamePattern);
    else queryParams.delete('username');

    if (filters.includeMyReports) queryParams.set('mine', filters.includeMyReports.toString());
    queryParams.set('page', page.toString());

    navigate({ search: queryParams.toString() });
  }, [filters, debouncedTitlePattern, page, navigate, location.search]);

  const { data, isLoading, error } = useQuery({
    queryKey: ['reports', location.search],
    queryFn: () => fetchReports(new URLSearchParams(location.search)),
  });

  const handleFilterChange = (field: keyof ReportQuery, value: string) => {
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
      page: 1,
      perPage: 30,
      userNamePattern: '',
      includeMyReports: true,
    });
    navigate({ search: '&per_page=30&page=1' });
  };

  const handleSort = (field: keyof Report) => {
    if (sortField === field) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortOrder('asc');
    }
  };

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

  const sortedReports = data?.reports ? [...data.reports].sort((a, b) => {
    const fieldA = a[sortField as keyof Report];
    const fieldB = b[sortField as keyof Report];

    if (fieldA < fieldB) {
      return sortOrder === 'asc' ? -1 : 1;
    }
    if (fieldA > fieldB) {
      return sortOrder === 'asc' ? 1 : -1;
    }
    return 0;
  }) : [];

  if (isLoading) {
    return <Loader />;
  }

  return (
    <Box style={{ width: '100%' }}>
      <Stack>
        <Group justify="center" style={{ marginBottom: '30px' }}>
          <Title order={1}>Browse Report</Title>
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
                  onClick={() => handleSort('analyses')}
                  justify={'space-between'}
                >
                  Analysis kinds
                  {renderSortIcon('analyses')}
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
                <Table.Td>{extractAnalysisKinds(report.analyses).join(', ')}</Table.Td>
                <Table.Td>
                  <Tooltip label={report.description} withArrow multiline w={300}>
                    <Box>{truncateText(report.description, 45)}</Box>
                  </Tooltip>
                </Table.Td>
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
          <Pagination total={data?.totalPages ?? 1} value={page} onChange={setPage} />
        </Center>
      </Stack>
    </Box>
  );
};


export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <BrowseReportsPage />
    </ErrorBoundary>
  );
}
