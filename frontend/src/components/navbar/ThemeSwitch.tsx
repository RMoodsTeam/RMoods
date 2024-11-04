import {ActionIcon, Menu, useMantineColorScheme} from "@mantine/core";
import {FaDesktop, FaMoon, FaSun} from "react-icons/fa";

  const ThemeSwitch = () => {
  const {colorScheme, setColorScheme} = useMantineColorScheme();
  const getIcon = () => {
    if (colorScheme == 'light') {
      return <FaSun size={18}/>
    } else if (colorScheme == 'dark') {
      return <FaMoon size={18}/>
    } else {
      return <FaDesktop size={18}/>
    }
  }

  return (
    <>
    <Menu>
      <Menu.Target>
        <ActionIcon size={36}>{getIcon()}</ActionIcon>
      </Menu.Target>
      <Menu.Dropdown>
        <Menu.Item onClick={() => setColorScheme('light')}>
          Light
        </Menu.Item>
        <Menu.Item onClick={() => setColorScheme('dark')}>
          Dark
        </Menu.Item>
        <Menu.Item onClick={() => setColorScheme('auto')}>
          System
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
    </>
  )
};

export default ThemeSwitch;
