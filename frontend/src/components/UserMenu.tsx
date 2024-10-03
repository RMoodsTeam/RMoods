import { Link, Menu, MenuButton, MenuItem, MenuList } from "@chakra-ui/react";
import Cookies from "js-cookie";

const UserMenu = () => {
  return (
    <Menu id="user-dropdown">
      <MenuButton>User Menu</MenuButton>
      <MenuList>
        <MenuItem>
          <Link href="/dashboard">Dashboard</Link>
        </MenuItem>
        <MenuItem>
          <Link href="/login" onClick={() => Cookies.remove("RMOODS_JWT")}>
            Log out
          </Link>
        </MenuItem>
      </MenuList>
    </Menu>
  );
};

export default UserMenu;
