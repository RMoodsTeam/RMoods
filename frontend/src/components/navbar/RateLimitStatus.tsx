import { useQuery, useQueryClient } from '@tanstack/react-query';
import authFetch from '../../rmoods/client/authFetch.ts';
import { Center, HoverCard, Loader, Progress, Stack, Text } from '@mantine/core';
import { IconBrandReddit } from '@tabler/icons-react';
import React, { useEffect } from 'react';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import duration from 'dayjs/plugin/duration';
import { ErrorBoundary } from 'react-error-boundary';
import { RateLimitStatusFallback } from '../fallbacks/RateLimitStatusFallback.tsx';

dayjs.extend(duration);
dayjs.extend(relativeTime);

/**
 * Mirrors the response from the Reddit API rate limit endpoint.
 * `backend/src/reddit_fetcher/reddit/ratelimit_headers.rs`
 */
export interface RatelimitResponse {
  remaining: number;
  reset: number;
  used: number;
  createdAt: number;
}

/**
 * The maximum number of requests allowed by the Reddit API in a 10-minute window.
 */
const MAX_REQUESTS = 1000;

/**
 * A hover card that displays the current Reddit API rate limit status.
 */
const RateLimitHoverCard = ({
  ratelimit,
}: {
  ratelimit: RatelimitResponse;
}) => {
  const [timestamp, setTimestamp] = React.useState(
    Math.floor(Date.now() / 1000)
  );

  // Update the timestamp every second - rerender the component
  useEffect(() => {
    const interval = setInterval(() => {
      setTimestamp(Math.floor(Date.now() / 1000));
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const remaining_percentage = (ratelimit.remaining / MAX_REQUESTS) * 100;
  const color = remaining_percentage > 30 ? 'blue' : 'red';

  const timeUntilReset = ratelimit.reset - (timestamp - ratelimit.createdAt);

  return (
    <Stack gap={8} style={{ padding: 4 }}>
      <Center>
        <Text>Reddit API Rate Limit</Text>
      </Center>
      {timeUntilReset > 0 ? (
        <Progress value={remaining_percentage} color={color} />
      ) : (
        <Progress value={100} color={color} />
      )}
      <Center>
        {timeUntilReset > 0 ? (
          <Text>
            {ratelimit.remaining} / {MAX_REQUESTS}
          </Text>
        ) : (
          <Text>
            {MAX_REQUESTS} / {MAX_REQUESTS}
          </Text>
        )}
      </Center>
      <Center>
        {timeUntilReset > 0 ? (
          timeUntilReset > 60 ? (
            <Text>{`Resetting in ${dayjs.duration(timeUntilReset, 'seconds').humanize()}`}</Text>
          ) : (
            <Text>{`Resetting in ${timeUntilReset} seconds`}</Text>
          )
        ) : (
          <></>
        )}
      </Center>
    </Stack>
  );
};

// Fetcher for Tanstack Query
const fetchRemainingRequests = async (): Promise<RatelimitResponse[]> => {
  const response = await authFetch(
    'http://localhost:8001/api/system/rate-limits'
  );
  return response.json();
};

const RateLimitStatus = () => {
  const client = useQueryClient();
  const query = useQuery({
    queryKey: ['remainingRequests'],
    queryFn: fetchRemainingRequests,
    refetchInterval: 1000,
  });

  if (query.isLoading) {
    return <Loader color="blue" size={20} />;
  }

  if (query.isError) {
    throw query.error;
  }

  client.setQueryData(['remainingRequests'], query.data);

  return (
    <HoverCard>
      <HoverCard.Target>
        <IconBrandReddit size={24} />
      </HoverCard.Target>
      <HoverCard.Dropdown>
        <RateLimitHoverCard ratelimit={query.data![0]} />
      </HoverCard.Dropdown>
    </HoverCard>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={RateLimitStatusFallback}>
      <RateLimitStatus />
    </ErrorBoundary>
  );
}
