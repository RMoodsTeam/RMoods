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

const router = createBrowserRouter([
  {
    element: (
      <ProtectedRoute>
        <DashboardLayout />
      </ProtectedRoute>
    ),
    children: [
      {
        path: 'RMoods/dashboard',
        element: <Dashboard />,
      },
      {
        path: 'RMoods/report',
        element: <Report />,
      },
      {
        path: 'RMoods/user',
        element: <UserPage />,
      },
    ],
  },
  {
    element: <Layout />,
    children: [
      {
        path: '/RMoods',
        element: <Root />,
      },
      {
        path: 'RMoods/about',
        element: <About />,
      },

      {
        path: 'RMoods/login',
        element: <Login />,
      },
    ],
  },
]);

export default router;
