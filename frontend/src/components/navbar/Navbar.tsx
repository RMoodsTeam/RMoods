import UserMenu from './UserMenu';
import ThemeSwitch from './ThemeSwitch';
import { Anchor, Burger, Card, Flex, Group } from '@mantine/core';
import StatusIndicator from './StatusIndicator';
import RateLimitStatus from './RateLimitStatus.tsx';
import { Link } from 'react-router-dom';

interface NavbarProps {
  onSidebarOpen?: () => void;
}

/**
 * Navbar contains components to navigate the website
 */
const Navbar = ({ onSidebarOpen }: NavbarProps) => {
  return (
    <Card
      style={{ margin: 0, borderRadius: 0, marginBottom: 0, height: '5em' }}
    >
      <nav>
        <Group justify="space-between">
          <Flex gap={10}>
            <Group>
              <Burger onClick={onSidebarOpen} hiddenFrom={'md'} />
              <Anchor component={Link} to="/">
                Main
              </Anchor>
              <Anchor component={Link} to="/about">
                About
              </Anchor>
              <Anchor component={Link} to="/report">
                Report
              </Anchor>
            </Group>
          </Flex>

          <Flex gap={10} justify={'right'} align="center">
            <RateLimitStatus />
            <StatusIndicator />
            <ThemeSwitch />
            <UserMenu />
          </Flex>
        </Group>
      </nav>
    </Card>
  );
};

export default Navbar;
