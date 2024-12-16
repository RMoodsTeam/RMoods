import { useState } from 'react';
import {
  Box,
  Collapse,
  Group,
  rem,
  Text,
  ThemeIcon,
  UnstyledButton,
} from '@mantine/core';
import { IconChevronRight } from '@tabler/icons-react';
import classes from './SidebarLinksGroup.module.css';
import { Link } from 'react-router-dom';

export interface LinksGroupProps {
  icon: React.FC<any>;
  label: string;
  initiallyOpened?: boolean;
  links?: { label: string; link: string }[];
  link?: string
}

export function LinksGroup({
  icon: Icon,
  label,
  initiallyOpened,
  links,
  link
}: LinksGroupProps) {
  const hasLinks = Array.isArray(links);
  const [opened, setOpened] = useState(initiallyOpened || false);
  const items = (hasLinks ? links : []).map((link) => (
    <Text
      component={Link}
      className={classes.link}
      to={link.link}
      key={link.label}
    >
      {link.label}
    </Text>
  ));

  return (
    <>
      <UnstyledButton
        onClick={() => setOpened((o) => !o)}
        className={classes.control}
      >
        {
          hasLinks?
          <>
            <Group justify="space-between" gap={0}>
              <Box style={{ display: 'flex', alignItems: 'center' }}>
                <ThemeIcon variant="light" size={30}>
                  <Icon style={{ width: rem(18), height: rem(18) }} />
                </ThemeIcon>
                <Box ml="md">{label}</Box>
              </Box>
              {hasLinks && (
                <IconChevronRight
                  className={classes.chevron}
                  stroke={1.5}
                  style={{
                    width: rem(16),
                    height: rem(16),
                    transform: opened ? 'rotate(90deg)' : 'none',
                  }}
                />
              )}
            </Group>
          {hasLinks ? <Collapse in={opened}>{items}</Collapse> : null}
          </> 
          : (
            <Text component={Link} className={classes.link} to={link}>label</Text>
          )
      }
      </UnstyledButton>
    </>
  );
}
