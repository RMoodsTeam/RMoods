import { Menu, MenuButton, MenuItem, MenuList } from "@chakra-ui/react";
import Cookies from "js-cookie";
import { useNavigate } from "react-router-dom";

const UserMenu = () => {
  const navigate = useNavigate();
  const handleLogout = () => {
    console.log("Logging out");
    Cookies.remove("RMOODS_JWT");
    navigate("/login");
  };

  return (
    <Menu id="user-dropdown">
      <MenuButton>User Menu</MenuButton>
      <MenuList>
        <MenuItem onClick={() => navigate("/dashboard")}>Dashboard</MenuItem>
        <MenuItem onClick={() => handleLogout()}>Log out</MenuItem>
      </MenuList>
    </Menu>
  );
};

export default UserMenu;
