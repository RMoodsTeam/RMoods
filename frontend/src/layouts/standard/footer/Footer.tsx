import { Group, Anchor, Box } from '@mantine/core';
import {
  IconBrandGithub,
  IconBrandTwitter,
  IconBrandYoutube,
} from '@tabler/icons-react';
import TemporaryLogo from './TemporaryLogo.tsx';
import classes from '../../shared/FooterCentered.module.scss';
import { Link, useNavigate } from 'react-router-dom';
import ActionIconComponent from '../../shared/actionIconComponent/ActionIconComponent.tsx';

const links = [
  { link: '/about', label: 'About' },
  { link: '/dashboard', label: 'Dashboard' },
];

export default function FooterCentered() {
  const navigate = useNavigate();
  const items = links.map((link) => (
    <Anchor
      c="dimmed"
      key={link.label}
      lh={1}
      onClick={(event) => {
        event.preventDefault();
        navigate(link.link);
      }}
      size="sm"
    >
      {link.label}
    </Anchor>
  ));

  return (
    <footer className={classes.footer}>
      <Box className={classes.inner}>
        <TemporaryLogo size={28} />

        <Group className={classes.links}>{items}</Group>

        <Group gap="xs" justify="flex-end" wrap="nowrap">
          <Anchor
            component={Link}
            to="https://github.com/RMoodsTeam/RMoods"
            target="_blank"
            rel="noopener noreferrer"
          >
            <ActionIconComponent icon={IconBrandGithub} />
          </Anchor>
          <Anchor
            component={Link}
            to="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
            target="_blank"
            rel="noopener noreferrer"
          >
            <ActionIconComponent icon={IconBrandTwitter} />
          </Anchor>
          <Anchor
            component={Link}
            to="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
            target="_blank"
            rel="noopener noreferrer"
          >
            <ActionIconComponent icon={IconBrandYoutube} />
          </Anchor>
        </Group>
      </Box>
    </footer>
  );
}
