import Sidebar from '../sidebar/Sidebar.tsx';
import { Outlet } from 'react-router-dom';
import Navbar from '../navbar/dashboardNavbar/Navbar.tsx';
import DashboardFooter from '../footer/Footer.tsx';
import { Flex } from '@mantine/core';
import { ScrollToTop } from '../../shared/ScrollToTop.tsx';
import { useDisclosure } from '@mantine/hooks';
import classes from './DashboardLayout.module.scss';

const dashboardContainer = {
  marginX: '5%',
  marginY: '2em',
  padding: '2rem 12%',
};

const DashboardLayout = () => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <Flex>
        <Sidebar opened={opened} onClose={close} />
        <Flex className={classes.outer}>
          <Navbar onSidebarOpen={open} />
          <Flex className={classes.container}>
            <Outlet />
          </Flex>
          <DashboardFooter />
        </Flex>
      </Flex>
      <ScrollToTop />
    </>
  );
};

export default DashboardLayout;
