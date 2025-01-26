import { TablerIcon } from '@tabler/icons-react';
import { ActionIcon } from '@mantine/core';
import classes from './ActionIconComponent.module.scss';

interface ActionIconComponentProps {
  icon: TablerIcon;
}

export default function ActionIconComponent({
  icon: Icon,
}: ActionIconComponentProps) {
  return (
    <ActionIcon size="lg" variant="default" radius="xl">
      <Icon className={classes.icon} stroke={1.5} />
    </ActionIcon>
  );
}
