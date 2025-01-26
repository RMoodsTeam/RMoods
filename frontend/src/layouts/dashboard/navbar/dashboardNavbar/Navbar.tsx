import UserMenu from '../userMenu/UserMenu.tsx';
import ThemeSwitch from '../../../shared/ThemeSwitch.tsx';
import { Anchor, Burger, Card, Flex, Group } from '@mantine/core';
import StatusIndicator from '../statusIndicator/StatusIndicator.tsx';
import RateLimitStatus from '../rateLimitStatus/RateLimitStatus.tsx';
import { Link } from 'react-router-dom';
import classes from './Navbar.module.scss';

interface NavbarProps {
  onSidebarOpen?: () => void;
}

/**
 * Navbar contains components to navigate the website
 */
const Navbar = ({ onSidebarOpen }: NavbarProps) => {
  return (
    <Card className={classes.card}>
      <nav>
        <Group justify="space-between">
          <Flex gap={10}>
            <Group>
              <Burger
                id="sidebar-burger"
                onClick={onSidebarOpen}
                hiddenFrom={'md'}
              />
              <Anchor component={Link} to="/" id="main-button">
                Main
              </Anchor>
              <Anchor component={Link} to="/about/faq">
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
