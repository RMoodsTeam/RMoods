import {Code, Group, ScrollArea} from '@mantine/core';
import classes from './NavbarNested.module.css';

export default function Navbar() {

  return (
    <nav className={classes.navbar} style={{border: '1px solid red'}}>
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