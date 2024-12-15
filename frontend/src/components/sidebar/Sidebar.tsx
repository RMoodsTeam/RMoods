import { Box, Code, Drawer, Group, ScrollArea } from '@mantine/core';
import {
  IconAdjustments,
  IconCalendarStats,
  IconFileAnalytics,
  IconGauge,
  IconLock,
  IconNotes,
  IconPresentationAnalytics,
} from '@tabler/icons-react';
import classes from './SidebarNested.module.scss';
import { LinksGroup } from './SidebarLinksGroup.tsx';

const mockdata = [
  { label: 'Dashboard', icon: IconGauge },
  {
    label: 'Market news',
    icon: IconNotes,
    initiallyOpened: true,
    links: [
      { label: 'TODO1', link: '/' },
      { label: 'TODO2', link: '/' },
      { label: 'TODO3', link: '/' },
      { label: 'TODO4', link: '/' },
    ],
  },
  {
    label: 'Releases',
    icon: IconCalendarStats,
    links: [
      { label: 'TODO5', link: '/' },
      { label: 'TODO6', link: '/' },
      { label: 'TODO7', link: '/' },
    ],
  },
  { label: 'Analytics', icon: IconPresentationAnalytics },
  { label: 'Contracts', icon: IconFileAnalytics },
  { label: 'Settings', icon: IconAdjustments },
  {
    label: 'Security',
    icon: IconLock,
    links: [
      { label: 'TODO7', link: '/' },
      { label: 'TODO8', link: '/' },
      { label: 'TODO9', link: '/' },
    ],
  },
];

const DesktopSidebar = () => {
  const links = mockdata.map((item) => (
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
  const links = mockdata.map((item) => (
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
