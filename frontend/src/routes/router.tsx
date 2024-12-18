import { createHashRouter } from 'react-router-dom';
import About from './about/page.tsx';
import Dashboard from './dashboard/page.tsx';
import Login from './login/page.tsx';
import Root from './page.tsx';
import UserPage from './user/userPage/page.tsx';
import Report from './report/page.tsx';
import Layout from '../layouts/standard/layout/Layout.tsx';
import ProtectedRoute from './ProtectedRoute.tsx';
import DashboardLayout from '../layouts/dashboard/layout/DashboardLayout.tsx';
import Settings from './settings/page.tsx';
import WebsocketProvider from '../providers/WebsocketProvider.tsx';

const router = createHashRouter([
  {
    element: (
      <WebsocketProvider>
        <ProtectedRoute>
          <DashboardLayout />
        </ProtectedRoute>
      </WebsocketProvider>
    ),
    children: [
      {
        path: '/dashboard',
        element: <Dashboard />,
      },
      {
        path: '/report',
        element: <Report />,
      },
      {
        path: '/user',
        element: <UserPage />,
      },
      {
        path: '/settings',
        element: <Settings />,
      },
    ],
  },
  {
    element: <Layout />,
    children: [
      {
        path: '/',
        element: <Root />,
      },
      {
        path: '/about',
        element: <About />,
      },
      {
        path: '/login',
        element: <Login />,
      },
    ],
  },
]);

export default router;
