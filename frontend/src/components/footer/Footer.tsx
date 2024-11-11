import { Anchor, Group, ActionIcon, rem } from '@mantine/core';
import { IconBrandGithub, IconBrandTwitter, IconBrandYoutube} from '@tabler/icons-react';
import TemporaryLogo from './TemporaryLogo';
import classes from './FooterCentered.module.css';

const links = [
  { link: '#', label: 'About' },
  { link: '#', label: 'Dashboard' }
];

export default function FooterCentered() {
  const items = links.map((link) => (
    <Anchor
      c="dimmed"
      key={link.label}
      href={link.link}
      lh={1}
      onClick={(event) => event.preventDefault()}
      size="sm"
    >
      {link.label}
    </Anchor>
  ));

  return (
    <footer className={classes.footer}>
      <div className={classes.inner}>
      <TemporaryLogo size={28} />

        <Group className={classes.links}>{items}</Group>

        <Group gap="xs" justify="flex-end" wrap="nowrap">
        <Anchor href="https://github.com/RMoodsTeam/RMoods" target="_blank" rel="noopener noreferrer">
          <ActionIcon size="lg" variant="default" radius="xl">
            <IconBrandGithub style={{ width: rem(18), height: rem(18) }} stroke={1.5} />
          </ActionIcon>
        </Anchor>
        <Anchor href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" target="_blank" rel="noopener noreferrer">
          <ActionIcon size="lg" variant="default" radius="xl">
            <IconBrandTwitter style={{ width: rem(18), height: rem(18) }} stroke={1.5} />
          </ActionIcon>
        </Anchor>
          <Anchor href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" target="_blank" rel="noopener noreferrer">
            <ActionIcon size="lg" variant="default" radius="xl">
              <IconBrandYoutube style={{ width: rem(18), height: rem(18) }} stroke={1.5} />
            </ActionIcon>
          </Anchor>
        </Group>
      </div>
    </footer>
  );
}