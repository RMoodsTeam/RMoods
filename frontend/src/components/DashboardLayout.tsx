import Sidebar from './sidebar/Sidebar.tsx';
import { Outlet } from 'react-router-dom';
import Navbar from './navbar/Navbar.tsx';
import DashboardFooter from './footer/DashboardFooter';
import { Box, Flex } from '@mantine/core';
import { ScrollToTop } from './ScrollToTop.tsx';
import { useDisclosure } from '@mantine/hooks';

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
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <Flex>
        <Sidebar opened={opened} onClose={close} />
        <Flex
          // TODO: Cleanup
          style={{
            flex: 'auto',
            flexDirection: 'column',
          }}
        >
          <Navbar onSidebarOpen={open} />
          <Box style={dashboardContainer} flex={1} display={'flex'}>
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
