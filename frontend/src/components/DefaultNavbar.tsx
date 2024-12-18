import { Anchor, Card, Flex, Grid, Group } from '@mantine/core';
import ThemeSwitch from './navbar/ThemeSwitch.tsx';
import { Link } from 'react-router-dom';

const LeftNavItems = () => {
  return (
    <Flex gap={10}>
      <Anchor component={Link} to="/" id="main-button">
        Main
      </Anchor>
      <Anchor component={Link} to="/about">
        About
      </Anchor>
      <Anchor component={Link} to="/dashboard">
        Dashboard
      </Anchor>
    </Flex>
  );
};

const RightNavItems = () => {
  return (
    <Flex gap={10} justify="right" align="center">
      <ThemeSwitch />
    </Flex>
  );
};

/**
 * Navbar contains components to navigate the website
 */
const DefaultNavbar = () => {
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

export default DefaultNavbar;
