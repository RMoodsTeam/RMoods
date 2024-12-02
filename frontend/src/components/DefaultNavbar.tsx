import { Anchor, Card, Flex, Grid } from '@mantine/core';
import ThemeSwitch from './navbar/ThemeSwitch.tsx';
import UserMenu from './navbar/UserMenu.tsx';

const LeftNavItems = () => {
  return (
    <Flex gap={10}>
      <Anchor href="/">Main</Anchor>
      <Anchor href="/about">About</Anchor>
    </Flex>
  );
};

const RightNavItems = () => {
  return (
    <Flex gap={10} justify={'right'} align="center">
      <ThemeSwitch />
      <UserMenu />
    </Flex>
  );
};

/**
 * Navbar contains components to navigate the website
 */
const DefaultNavbar = () => {
  return (
    <Card
      style={{ margin: 0, borderRadius: 0, marginBottom: 0, height: '10vh' }}
    >
      <nav>
        <Grid>
          <Grid.Col span={4}>
            <LeftNavItems />
          </Grid.Col>
          <Grid.Col span={4}>
            <div />
          </Grid.Col>
          <Grid.Col span={4}>
            <RightNavItems />
          </Grid.Col>
        </Grid>
      </nav>
    </Card>
  );
};

export default DefaultNavbar;
