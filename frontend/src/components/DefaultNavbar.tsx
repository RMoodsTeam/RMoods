import { Anchor, Card, Flex, Grid, Group } from '@mantine/core';
import ThemeSwitch from './navbar/ThemeSwitch.tsx';

const LeftNavItems = () => {
  return (
    <Flex gap={10}>
      <Anchor href="/">Main</Anchor>
      <Anchor href="/about">About</Anchor>
      <Anchor href="/dashboard">Dashboard</Anchor>
    </Flex>
  );
};

const RightNavItems = () => {
  return (
    <Flex gap={10} justify={'right'} align="center">
      <ThemeSwitch />
    </Flex>
  );
};

/**
 * Navbar contains components to navigate the website
 */
const DefaultNavbar = () => {
  return (
    <Card
      style={{ margin: 0, borderRadius: 0, marginBottom: 0 }}
    >
      <nav>
        <Group justify='space-between'>
          <LeftNavItems />
          <RightNavItems />
        </Group>
      </nav>
    </Card>
  );
};

export default DefaultNavbar;
