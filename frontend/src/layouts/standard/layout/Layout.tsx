import { Outlet } from 'react-router-dom';
import Footer from '../footer/Footer.tsx';
import { Flex } from '@mantine/core';
import Navbar from '../navbar/Navbar.tsx';
import classes from './Layout.module.scss';

const Layout = () => {
  return (
    <Flex className={classes.outer}>
      <Navbar />
      <Flex className={classes.inner}>
        <Outlet />
      </Flex>
      <Footer />
    </Flex>
  );
};

export default Layout;
