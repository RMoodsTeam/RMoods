import { createBrowserRouter } from "react-router-dom";
import App from "./App";
import About from "./routes/about/page";
import Dashboard from "./routes/dashboard/page";
import Login from "./routes/login/page";
import Layout from "./components/Layout";

const router = createBrowserRouter([
  {
    path: "/",
    element: (
      <Layout>
        <App />
      </Layout>
    ),
  },
  {
    path: "/about",
    element: (
      <Layout>
        <About />
      </Layout>
    ),
  },
  {
    path: "/dashboard",
    element: (
      <Layout>
        <Dashboard />
      </Layout>
    ),
  },
  {
    path: "/login",
    element: (
      <Layout>
        <Login />
      </Layout>
    ),
  },
]);

export default router;
