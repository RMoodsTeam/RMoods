import { Navigate } from "react-router-dom";
import Cookies from "js-cookie";

const ProtectedRoute = ({ children }: { children: React.ReactNode }) => {
  const jwt = Cookies.get("RMOODS_JWT");

  if (!jwt) {
    return <Navigate to="/login" replace />;
  }

  return children;
};

export default ProtectedRoute;
