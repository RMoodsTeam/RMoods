import { Link, Menu, MenuButton, MenuItem } from "@chakra-ui/react";
import Cookies from "js-cookie";

const UserMenu = () => {
  return (
    <Menu id="user-dropdown">
      <MenuButton>User Menu</MenuButton>
      <Menu placement="bottom-end">
        <MenuItem>
          <Link href="/dashboard">Dashboard</Link>
        </MenuItem>
        <MenuItem>
          <Link href="/login" onClick={() => Cookies.remove("RMOODS_JWT")}>
            Log out
          </Link>
        </MenuItem>
      </Menu>
    </Menu>
  );
};

export default UserMenu;
