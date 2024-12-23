import { createHashRouter } from 'react-router-dom';
import Faq from './about/faq/page.tsx';
import Dashboard from './dashboard/page.tsx';
import Login from './login/page.tsx';
import Root from './page.tsx';
import UserPage from './user/page.tsx';
import Report from './report/page/page.tsx';
import Layout from '../layouts/standard/layout/Layout.tsx';
import ProtectedRoute from './ProtectedRoute.tsx';
import DashboardLayout from '../layouts/dashboard/layout/DashboardLayout.tsx';
import Settings from './settings/page.tsx';
import WebsocketProvider from '../providers/WebsocketProvider.tsx';
import Releases from './releases/page.tsx';
import Thesis from './about/thesis/page.tsx';
import BrowseReports from './browse/reports/page.tsx';
import BrowseUsers from './browse/users/page.tsx';
import Sandbox from './sandbox/page.tsx';
import UserReports from './user/reports/page.tsx';
import RMoodsProvider from '../providers/RMoodsProvider.tsx';

const router = createHashRouter([
  {
    element: (
      <WebsocketProvider>
        <ProtectedRoute>
          <RMoodsProvider>
            <DashboardLayout />
          </RMoodsProvider>
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
        path: '/user/reports',
        element: <UserReports />,
      },
      {
        path: '/settings',
        element: <Settings />,
      },
      {
        path: '/browse/reports',
        element: <BrowseReports />,
      },
      {
        path: '/browse/users',
        element: <BrowseUsers />,
      },
      {
        path: '/sandbox',
        element: <Sandbox />,
      },
      {
        path: '/releases',
        element: <Releases />,
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
        path: '/about/faq',
        element: <Faq />,
      },
      {
        path: '/about/thesis',
        element: <Thesis />,
      },
      {
        path: '/login',
        element: <Login />,
      },
    ],
  },
]);

export default router;
