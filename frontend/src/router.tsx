import { createBrowserRouter } from 'react-router-dom';
import About from './routes/about/page';
import Dashboard from './routes/dashboard/page';
import Login from './routes/login/page';
import Root from './routes/page';
import Layout from './Layout';
import ProtectedRoute from './components/ProtectedRoute';
import DashboardLayout from './components/DashboardLayout.tsx';
import Report from './routes/report/page';

const router = createBrowserRouter([
  {
    element: <DashboardLayout />,
    children: [
      {
        path: 'dashboard',
        element: (
          <ProtectedRoute>
            <Dashboard />
          </ProtectedRoute>
        ),
      },
      {
        path: 'report',
        element: (
          <ProtectedRoute>
            <Report />
          </ProtectedRoute>
        ),
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
