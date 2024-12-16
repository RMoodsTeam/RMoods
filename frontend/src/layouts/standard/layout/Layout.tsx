import { Outlet } from 'react-router-dom';
import Footer from '../footer/Footer.tsx';
import { Box, Flex } from '@mantine/core';
import Navbar from '../navbar/Navbar.tsx';

const Layout = () => {
  return (
    <Flex
      style={{
        minHeight: '100vh',
        flexDirection: 'column',
        transition: 'background-color 0.5s ease',
      }}
    >
      <Navbar />
      <Flex
        style={{
          // flex 1 to fill the entire available space
          flex: '1',
          padding: '2rem 15%',
        }}
      >
        <Outlet />
      </Flex>
      <Footer />
    </Flex>
  );
};

export default Layout;
