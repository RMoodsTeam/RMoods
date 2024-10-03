import { createBrowserRouter } from "react-router-dom";
import About from "./routes/about/page";
import Dashboard from "./routes/dashboard/page";
import Login from "./routes/login/page";
import Layout from "./components/Layout";
import Root from "./routes/page";

const router = createBrowserRouter([
  {
    element: <Layout />,
    children: [
      {
        path: "/",
        element: <Root />,
      },
      {
        path: "about",
        element: <About />,
      },
      {
        path: "dashboard",
        element: <Dashboard />,
      },
      {
        path: "login",
        element: <Login />,
      },
    ],
  },
]);

export default router;
