import UserMenu from './UserMenu';
import ThemeSwitch from './ThemeSwitch';
import { Card, Flex, Grid } from '@mantine/core';
import StatusIndicator from './StatusIndicator';
import { Link } from 'react-router-dom';

const LeftNavItems = () => {
  return (
    <Flex gap={10}>
      <Link to="/">Main</Link >
      <Link to="/about">About</Link>
    </Flex>
  );
};

const RightNavItems = () => {
  return (
    <Flex gap={10} justify={'right'} align="center">
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

export default Navbar;
