import { createBrowserRouter } from 'react-router-dom';
import About from './routes/about/page';
import Dashboard from './routes/dashboard/page';
import Login from './routes/login/page';
import Root from './routes/page';
import UserPage from './routes/user/page';
import Report from './routes/report/page';
import Layout from './Layout';
import ProtectedRoute from './components/ProtectedRoute';
import DashboardLayout from './components/DashboardLayout.tsx';
import Settings from './routes/settings/page.tsx';
import WebsocketProvider from './providers/WebsocketProvider.tsx';

const router = createBrowserRouter([
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
        path: 'dashboard',
        element: <Dashboard />,
      },
      {
        path: 'report',
        element: <Report />,
      },
      {
        path: 'user',
        element: <UserPage />,
      },
      {
        path: 'settings',
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
        path: 'about',
        element: <About />,
      },
      {
        path: 'login',
        element: <Login />,
      },
    ],
  },
]);

export default router;
