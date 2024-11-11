import {Code, Group, ScrollArea} from '@mantine/core';
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
import {LinksGroup} from "./SidebarLinksGroup.tsx";

const mockdata = [
  { label: 'Dashboard', icon: IconGauge },
  {
    label: 'Market news',
    icon: IconNotes,
    initiallyOpened: true,
    links: [
      { label: 'Overview', link: '/' },
      { label: 'Forecasts', link: '/' },
      { label: 'Outlook', link: '/' },
      { label: 'Real time', link: '/' },
    ],
  },
  {
    label: 'Releases',
    icon: IconCalendarStats,
    links: [
      { label: 'Upcoming releases', link: '/' },
      { label: 'Previous releases', link: '/' },
      { label: 'Releases schedule', link: '/' },
    ],
  },
  { label: 'Analytics', icon: IconPresentationAnalytics },
  { label: 'Contracts', icon: IconFileAnalytics },
  { label: 'Settings', icon: IconAdjustments },
  {
    label: 'Security',
    icon: IconLock,
    links: [
      { label: 'Enable 2FA', link: '/' },
      { label: 'Change password', link: '/' },
      { label: 'Recovery codes', link: '/' },
    ],
  },
];


export default function Sidebar() {
  const links = mockdata.map((item) => <LinksGroup {...item} key={item.label} />);

// TODO: extract the styles to a variable
  return (
    <nav className={classes.navbar} style={{
      margin: 0,
      minWidth: "300px",
      maxWidth: "300px",
      height: "100vh",
      zIndex: 1,
      overflow: "hidden",
      borderRadius: 0,
      position: "sticky",
      top: 0,
      padding: 0,
    }}>
      <div className={classes.header}>
        <Group justify="space-between">
          <Code fw={700}>RMoods</Code>
        </Group>
      </div>

      <ScrollArea>
        <div className={classes.linksInner}>{links}</div>
      </ScrollArea>

      <div className={classes.footer}>
      </div>
    </nav>
  );
}