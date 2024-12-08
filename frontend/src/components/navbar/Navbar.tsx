import UserMenu from './UserMenu';
import ThemeSwitch from './ThemeSwitch';
import { Anchor, Card, Flex, Group } from '@mantine/core';
import StatusIndicator from './StatusIndicator';
import RateLimitStatus from './RateLimitStatus.tsx';
import { ErrorBoundary } from 'react-error-boundary';
import { RateLimitStatusFallback } from '../fallbacks/RateLimitStatusFallback.tsx';

const LeftNavItems = () => {
  return (
    <Flex gap={10}>
      <Anchor href="/">Main</Anchor>
      <Anchor href="/about">About</Anchor>
      <Anchor href="/report">Report</Anchor>
    </Flex>
  );
};

const RightNavItems = () => {
  return (
    <Flex gap={10} justify={'right'} align="center">
      <ErrorBoundary fallback={<RateLimitStatusFallback />}>
        <RateLimitStatus />
      </ErrorBoundary>
      <StatusIndicator />
      <ThemeSwitch />
      <UserMenu />
    </Flex>
  );
};

/**
 * Navbar contains components to navigate the website
 */
const Navbar = () => {
  return (
    <Card style={{ margin: 0, borderRadius: 0, marginBottom: 0 }}>
      <nav>
        <Group justify="space-between">
          <LeftNavItems />
          <RightNavItems />
        </Group>
      </nav>
    </Card>
  );
};

export default Navbar;
