import { Outlet } from "react-router-dom";
import Navbar from "./components/navbar/Navbar";
import MainContainer from "./components/MainContainer";
import Footer from "./components/Footer";

const Layout = () => {
  return (
    <div
      style={{
        minHeight: "100vh",
        position: "relative",
      }}
    >
      <Navbar />
      <MainContainer>
        <Outlet />
      </MainContainer>
      <Footer />
    </div>
  );
};

export default Layout;
