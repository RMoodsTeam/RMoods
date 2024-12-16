import { Box, Code, Drawer, Group, ScrollArea } from '@mantine/core';
import {
  IconAdjustments,
  IconCalendarStats,
  IconGauge,
  IconInfoCircle,
  IconPlus,
  IconReport,
  IconSandbox,
  IconSearch,
} from '@tabler/icons-react';
import classes from './SidebarNested.module.scss';
import { LinksGroup, LinksGroupProps } from './SidebarLinksGroup.tsx';
import React from 'react';

const sidebarItems: LinksGroupProps[] = [
  { label: 'Dashboard', icon: IconGauge, link: '/dashboard' },
  { label: 'New Report', icon: IconPlus, link: '/report' },
  {
    label: 'My Reports',
    icon: IconReport,
    link: '/myreports',
  },
  {
    label: 'Browse',
    icon: IconSearch,
    initiallyOpened: true,
    links: [
      { label: 'Reports', link: '/' },
      { label: 'Users', link: '/' },
    ],
  },
  {
    label: 'NLP Sandbox',
    icon: IconSandbox,
  },
  { label: 'Settings', icon: IconAdjustments },
  {
    label: 'Releases',
    icon: IconCalendarStats,
  },
  {
    label: 'About',
    icon: IconInfoCircle,
    links: [
      { label: 'FAQ', link: '/' },
      { label: 'Thesis', link: '/' },
    ],
  },
];

const DesktopSidebar = () => {
  const links = sidebarItems.map((item) => (
    <LinksGroup {...item} key={item.label} />
  ));

  // TODO: extract the styles to a variable
  return (
    <Box visibleFrom={'md'}>
      <nav
        className={classes.sidebar}
        style={{
          margin: 0,
          width: 250,
          height: '100vh',
          zIndex: 1,
          overflow: 'hidden',
          borderRadius: 0,
          position: 'sticky',
          top: 0,
          padding: 0,
        }}
      >
        <div className={classes.header}>
          <Group justify="space-between">
            <Code fw={700}>RMoods</Code>
          </Group>
        </div>

        <ScrollArea>
          <div className={classes.linksInner}>{links}</div>
        </ScrollArea>

        <div className={classes.footer}></div>
      </nav>
    </Box>
  );
};

interface MobileSidebarProps {
  opened: boolean;
  onClose: () => void;
}

const MobileSidebar = ({ opened, onClose }: MobileSidebarProps) => {
  const links = sidebarItems.map((item) => (
    <LinksGroup {...item} key={item.label} />
  ));

  // TODO: extract the styles to a variable
  return (
    <>
      <Drawer
        hiddenFrom={'md'}
        opened={opened}
        onClose={onClose}
        overlayProps={{ backgroundOpacity: 0.5, blur: 4 }}
        offset={8}
        radius={'lg'}
        size={'xs'}
        title={
          <Box className={classes.header}>
            <Group justify="space-between">
              <Code fw={700}>RMoods</Code>
            </Group>
          </Box>
        }
      >
        <nav className={classes.sidebar}>
          <ScrollArea>
            <div className={classes.linksInner}>{links}</div>
          </ScrollArea>

          <div className={classes.footer}></div>
        </nav>
      </Drawer>
      <Box id="mobile-height-placeholder" h="100vh" w={0}></Box>
    </>
  );
};

interface SidebarProps {
  opened: boolean;
  onClose: () => void;
}

export default function Sidebar({ opened, onClose }: SidebarProps) {
  return (
    <>
      <DesktopSidebar />
      <MobileSidebar opened={opened} onClose={onClose} />
    </>
  );
}
