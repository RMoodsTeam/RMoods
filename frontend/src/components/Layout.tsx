import { Outlet } from "react-router-dom";
import Footer from "./Footer";
import MainContainer from "./MainContainer";
import Navbar from "./Navbar";

const Layout = () => {
  return (
    <div style={{ minHeight: "100vh", position: "relative" }}>
      <Navbar />
      <MainContainer>
        <Outlet />
      </MainContainer>
      <Footer />
    </div>
  );
};

export default Layout;
