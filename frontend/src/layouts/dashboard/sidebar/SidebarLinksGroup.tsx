import { useState } from 'react';
import {
  Box,
  Collapse,
  Flex,
  Group,
  rem,
  Text,
  ThemeIcon,
  UnstyledButton,
} from '@mantine/core';
import { IconChevronRight } from '@tabler/icons-react';
import classes from './SidebarLinksGroup.module.scss';
import { Link } from 'react-router-dom';

export interface SidebarEntryProps {
  icon: React.FC<any>;
  label: string;
  initiallyOpened?: boolean;
  links?: { label: string; link: string }[];
  link?: string;
}

const LinksGroup = ({
  icon: Icon,
  initiallyOpened,
  label,
  links,
}: Omit<SidebarEntryProps, 'link'>) => {
  const [opened, setOpened] = useState(initiallyOpened || false);
  const hasLinks = Array.isArray(links) && links.length > 0;

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
        <Group justify="space-between" gap={0}>
          <Flex className={classes.entryFlex}>
            <ThemeIcon variant="light" size={30}>
              <Icon className={classes.icon} />
            </ThemeIcon>
            <Box ml="md">{label}</Box>
          </Flex>
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
      </UnstyledButton>
      {hasLinks ? <Collapse in={opened}>{items}</Collapse> : null}
    </>
  );
};

export function SidebarEntry({
  icon: Icon,
  label,
  initiallyOpened,
  links,
  link,
}: SidebarEntryProps) {
  if (link && links) throw new Error('Cannot have both link and links');

  if (link) {
    return (
      <Link to={link} className={`${classes.onlyLink} ${classes.control}`}>
        <Group justify="space-between" gap={0}>
          <Flex className={classes.entryFlex}>
            <ThemeIcon variant="light" size={30}>
              <Icon className={classes.icon} />
            </ThemeIcon>
            <Box ml="md">
              <Text>{label}</Text>
            </Box>
          </Flex>
        </Group>
      </Link>
    );
  } else {
    return (
      <LinksGroup
        icon={Icon}
        label={label}
        initiallyOpened={initiallyOpened}
        links={links}
      />
    );
  }
}
