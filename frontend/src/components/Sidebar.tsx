import {Code, Group, ScrollArea} from '@mantine/core';
import classes from './NavbarNested.module.css';

export default function Navbar() {

// TODO: extract the styles to a variable
  return (
    <nav className={classes.navbar} style={{
      margin: 0,
      minWidth: "200px",
      maxWidth: "200px",
      height: "100vh",
      zIndex: 1,
      overflow: "hidden",
      borderRadius: 0,
      position: "sticky",
      top: 0,
      paddingTop: 0,
    }}>
      <div className={classes.header}>
        <Group justify="space-between">
          <Code fw={700}>RMoods Logo</Code>
        </Group>
      </div>

      <ScrollArea className={classes.links}>
        <div className={classes.linksInner}>Sidebar Placeholder</div>
      </ScrollArea>

      <div className={classes.footer}>
      </div>
    </nav>
  );
}