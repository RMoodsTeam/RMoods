import { Code, Group, ScrollArea } from '@mantine/core';
import {
  IconNotes,
  IconCalendarStats,
  IconGauge,
  IconPresentationAnalytics,
  IconFileAnalytics,
  IconAdjustments,
  IconLock,
} from '@tabler/icons-react';
import classes from './SidebarNested.module.css';
import { LinksGroup } from './SidebarLinksGroup.tsx';
import RMoodsLogo from '.././RMoodsLogo.tsx';

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

export default function Sidebar() {
  const links = mockdata.map((item) => (
    <LinksGroup {...item} key={item.label} />
  ));

  // TODO: extract the styles to a variable
  return (
    <nav
      className={classes.navbar}
      style={{
        margin: 0,
        minWidth: '180px',
        maxWidth: '180px',
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
          <RMoodsLogo />
        </Group>
      </div>

      <ScrollArea>
        <div className={classes.linksInner}>{links}</div>
      </ScrollArea>

      <div className={classes.footer}></div>
    </nav>
  );
}
