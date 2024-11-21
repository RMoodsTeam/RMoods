import { Outlet } from 'react-router-dom';
import Navbar from './components/navbar/Navbar';
import Footer from './components/footer/Footer';
import { Flex } from '@mantine/core';

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
      <Outlet />
      <Footer />
    </Flex>
  );
};

export default Layout;
