import { Box, Text, Loader } from '@mantine/core';
import Cookies from 'js-cookie';
import { jwtDecode } from 'jwt-decode';
import UserCard from './UserCard';
import StatisticItem from './StatisticItem';
import { JwtClaims } from '../../rmoods/jwt.ts';
import authFetch from '../../rmoods/client/authFetch.ts';
import { useQuery } from '@tanstack/react-query';

/**
 * User interface representing the user data.
 */
export interface User {
  name: string;
  given_name: string;
  email: string;
  picture: string;
}

/**
 * Statistics data for the user.
 */
const statistics = {
  likedReports: ['Report 1', 'Report 2', 'Report 3'],
  latestReportDate: '2023-10-01',
  totalReports: 100,
  topSubreddits: ['subreddit1', 'subreddit2', 'subreddit3'],
  karma: 5000,
  totalRequests: 200,
  generatedCost: 150.75,
  longestReportTime: '2 hours',
};

const fetchUserData = async (): Promise<User> => {
  const token = Cookies.get('RMOODS_JWT');
  if (!token) {
    throw new Error('No JWT token found');
  }

  const data = jwtDecode<JwtClaims>(token);
  const id = data.userInfo.id;
  const response = await authFetch(`http://localhost:8001/api/user?id=${id}`);
  if (!response.ok) {
    throw new Error('Failed to fetch user data');
  }

  return response.json();
};

/**
 * UserPage component that displays the user's profile and statistics.
 * @returns {JSX.Element} The UserPage component.
 */
const UserPage = () => {
  const { data, error, isLoading } = useQuery<User, Error>({
    queryKey: ['userData'],
    queryFn: fetchUserData,
    refetchInterval: 5000,
  });

  if (isLoading) {
    return (
      <Box
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          height: '50vh',
        }}
      >
        <Loader color="blue" size={40} />
      </Box>
    );
  }

  if (error) {
    return <Text>Error: {error.message}</Text>;
  }

  return (
    <Box
      style={{
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'flex-start'
      }}
    >
      <Box style={{ flex: '0 0 350px', marginRight: '20px' }}>
        {data && <UserCard user={data} />}
      </Box>
      <Box style={{ flex: '1', display: 'flex', flexWrap: 'wrap' }}>
        <Box style={{ flex: '1 1 50%', padding: '10px' }}>
          <StatisticItem
            label='Liked reports'
            value={statistics.likedReports.join(', ')}
          />
          <StatisticItem
            label='Latest report'
            value={statistics.latestReportDate}
          />
          <StatisticItem
            label='Number of created reports'
            value={statistics.totalReports}
          />
          <StatisticItem
            label='Top 3 subreddits'
            value={statistics.topSubreddits.join(', ')}
          />
        </Box>
        <Box style={{ flex: '1 1 50%', padding: '10px' }}>
          <StatisticItem label='Karma' value={statistics.karma} />
          <StatisticItem
            label='Total cost of reports'
            value={statistics.generatedCost}
          />
          <StatisticItem
            label='Total used requests'
            value={statistics.totalRequests}
          />
          <StatisticItem
            label='Longest Report Time'
            value={statistics.longestReportTime}
          />
        </Box>
      </Box>
    </Box>
  );
};

export default UserPage;
