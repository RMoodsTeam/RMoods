import Cookies from "js-cookie";
import { useNavigate } from "react-router-dom";
import { Button, Menu } from "@mantine/core";

const UserMenu = () => {
  const navigate = useNavigate();
  const handleLogout = () => {
    console.log("Logging out");
    Cookies.remove("RMOODS_JWT");
    navigate("/login");
  };

  return (
    <Menu id="user-dropdown">
      <Menu.Target><Button>User Menu</Button></Menu.Target>
      <Menu.Dropdown>
        <Menu.Item onClick={() => navigate("/dashboard")}>Dashboard</Menu.Item>
        <Menu.Item onClick={() => navigate("/user")}>Profile</Menu.Item>
        <Menu.Item onClick={() => handleLogout()}>Log out</Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
};

export default UserMenu;
