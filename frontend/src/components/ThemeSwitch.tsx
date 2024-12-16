import {
  ActionIcon,
  MantineColorScheme,
  Menu,
  useMantineColorScheme,
} from '@mantine/core';
import { FaDesktop, FaMoon, FaSun } from 'react-icons/fa';
import { useAtom } from 'jotai';
import { colorModeAtom } from '../atoms.ts';

const ThemeSwitch = () => {
  const { colorScheme, setColorScheme } = useMantineColorScheme();
  const [, setColorModeAtom] = useAtom(colorModeAtom);

  const iconMap = {
    light: <FaSun size={18} />,
    dark: <FaMoon size={18} />,
    auto: <FaDesktop size={18} />,
  };

  const getIcon = () => {
    return iconMap[colorScheme];
  };

  const setStoredColorScheme = (newScheme: MantineColorScheme) => {
    setColorScheme(newScheme);
    setColorModeAtom(newScheme);
  };

  return (
    <>
      <Menu>
        <Menu.Target>
          <ActionIcon size={36}>{getIcon()}</ActionIcon>
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Item
            leftSection={iconMap['light']}
            onClick={() => setStoredColorScheme('light')}
          >
            Light
          </Menu.Item>
          <Menu.Item
            leftSection={iconMap['dark']}
            onClick={() => setStoredColorScheme('dark')}
          >
            Dark
          </Menu.Item>
          <Menu.Item
            leftSection={iconMap['auto']}
            onClick={() => setStoredColorScheme('auto')}
          >
            System
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>
    </>
  );
};

export default ThemeSwitch;
