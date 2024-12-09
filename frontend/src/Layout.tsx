import { Outlet } from 'react-router-dom';
import Footer from './components/footer/Footer';
import { Box, Flex } from '@mantine/core';
import DefaultNavbar from './components/DefaultNavbar.tsx';

const Layout = () => {
  return (
    <Flex
      style={{
        minHeight: '100vh',
        flexDirection: 'column',
        transition: 'background-color 0.5s ease',
      }}
    >
      <DefaultNavbar />
      <Box
        style={{
          padding: '2rem 15%',
        }}
      >
        <Outlet />
      </Box>
      <Footer />
    </Flex>
  );
};

export default Layout;
