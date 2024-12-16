import Sidebar from '../sidebar/Sidebar.tsx';
import { Outlet } from 'react-router-dom';
import Navbar from '../navbar/Navbar.tsx';
import DashboardFooter from '../footer/Footer.tsx';
import { Flex } from '@mantine/core';
import { ScrollToTop } from '../../../components/ScrollToTop.tsx';
import { useDisclosure } from '@mantine/hooks';

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
          <Flex style={dashboardContainer} flex={1}>
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
