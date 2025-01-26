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
import { SidebarEntry, SidebarEntryProps } from './SidebarLinksGroup.tsx';
import React from 'react';
import RMoodsLogo from '../../../components/RMoodsLogo.tsx';

const sidebarItems: SidebarEntryProps[] = [
  { label: 'Dashboard', icon: IconGauge, link: '/dashboard' },
  { label: 'New Report', icon: IconPlus, link: '/report' },
  {
    label: 'My Reports',
    icon: IconReport,
    link: '/user/reports',
  },
  {
    label: 'Browse',
    icon: IconSearch,
    initiallyOpened: true,
    links: [
      { label: 'Reports', link: '/browse/reports' },
      { label: 'Users', link: '/browse/users' },
    ],
  },
  {
    label: 'NLP Sandbox',
    icon: IconSandbox,
    link: '/sandbox',
  },
  {
    label: 'Settings',
    icon: IconAdjustments,
    link: '/settings',
  },
  {
    label: 'Releases',
    icon: IconCalendarStats,
    link: '/releases',
  },
  {
    label: 'About',
    icon: IconInfoCircle,
    links: [
      { label: 'FAQ', link: '/about/faq' },
      { label: 'Thesis', link: '/about/thesis' },
    ],
  },
];

const DesktopSidebar = () => {
  const links = sidebarItems.map((item) => (
    <SidebarEntry {...item} key={item.label} />
  ));

  return (
    <Box visibleFrom={'md'}>
      <nav className={classes.sidebar}>
        <div className={classes.header}>
          <Group justify="space-between">
            <RMoodsLogo />
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
    <SidebarEntry {...item} key={item.label} />
  ));

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
