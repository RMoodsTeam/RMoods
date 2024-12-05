import Sidebar from './sidebar/Sidebar.tsx';
import { Outlet } from 'react-router-dom';
import Navbar from './navbar/Navbar.tsx';
import DashboardFooter from './dashboardFooter/DashboardFooter.tsx';
import { Box, Flex } from '@mantine/core';
import { ScrollToTop } from '../components/ScrollToTop';

const dashboardFlex = {
  flex: 'auto',
  flexDirection: 'column',
};

const dashboardContainer = {
  marginX: '5%',
  marginY: '2em',
  padding: '2rem 12%',
};

const DashboardLayout = () => {
  return (
    <>
      <Flex>
        <Sidebar />
        <Flex
          // TODO: Cleanup
          style={{
            flex: 'auto',
            flexDirection: 'column',
          }}
        >
          <Navbar />
          <Box style={dashboardContainer}>
            <Outlet />
          </Box>
          <DashboardFooter />
        </Flex>
      </Flex>
      <ScrollToTop />
    </>
  );
};

export default DashboardLayout;
