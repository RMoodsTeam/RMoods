import Footer from "./Footer";
import MainContainer from "./MainContainer";
import Navbar from "./Navbar";

const Layout = ({ children }: any) => {
  return (
    <>
      <Navbar />
      <MainContainer>{children}</MainContainer>
      <Footer />
    </>
  );
};

export default Layout;
