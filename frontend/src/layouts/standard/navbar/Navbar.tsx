import { Anchor, Card, Flex, Group } from '@mantine/core';
import ThemeSwitch from '../../shared/ThemeSwitch.tsx';
import { Link } from 'react-router-dom';
import classes from './Navbar.module.scss';

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
const Navbar = () => {
  return (
    <Card className={classes.wrapper}>
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
