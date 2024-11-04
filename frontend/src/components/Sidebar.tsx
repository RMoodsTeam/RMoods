import { Group, Code, ScrollArea } from '@mantine/core';
import classes from './NavbarNested.module.css';

export default function Navbar() {

  return (
    <nav className={classes.navbar}>
      <div className={classes.header}>
        <Group justify="space-between">
          <Code fw={700}>RMoods!</Code>
        </Group>
      </div>

      <ScrollArea className={classes.links}>
        <div className={classes.linksInner}>Hello World!</div>
      </ScrollArea>

      <div className={classes.footer}>
      </div>
    </nav>
  );
}