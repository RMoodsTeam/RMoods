"use client";
import Dropdown from "@mui/joy/Dropdown";
import IconButton from "@mui/joy/IconButton";
import Menu from "@mui/joy/Menu";
import MenuItem from "@mui/joy/MenuItem";
import ListItemDecorator from "@mui/joy/ListItemDecorator";
import MenuButton from "@mui/joy/MenuButton";
import { Link } from "@mui/joy";
import Cookies from "js-cookie";

const UserMenu = () => {
  return (
    <Dropdown id="user-dropdown">
      <MenuButton
        slots={{ root: IconButton }}
        slotProps={{ root: { variant: "outlined", color: "neutral" } }}
      >
        User Menu
      </MenuButton>
      <Menu placement="bottom-end">
        <MenuItem>
          <ListItemDecorator />
          <Link href="/dashboard">Dashboard</Link>
        </MenuItem>
        <MenuItem>
          <ListItemDecorator />
          <Link href="/login" onClick={() => Cookies.remove("RMOODS_JWT")}>
            Log out
          </Link>
        </MenuItem>
      </Menu>
    </Dropdown>
  );
};

export default UserMenu;
