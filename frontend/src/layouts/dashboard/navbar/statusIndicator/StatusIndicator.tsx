import React from 'react';
import { useAtomValue } from 'jotai';
import { ActionIcon, Tooltip } from '@mantine/core';
import { wsConnectionStatusAtom } from '../../../../atoms.ts';
import classes from './StatusIndicator.module.scss';

const StatusIndicator = () => {
  const isConnected = useAtomValue(wsConnectionStatusAtom);

  const tooltipLabel = isConnected
    ? `Connected to the RMoods Server`
    : `Disconnected from the RMoods Server`;

  return (
    <Tooltip label={tooltipLabel} openDelay={300}>
      <ActionIcon
        size="xs"
        radius="100%"
        color={isConnected ? 'green' : 'red'}
        className={classes.icon}
      />
    </Tooltip>
  );
};

export default StatusIndicator;
