import React from 'react';
import { useAtomValue } from 'jotai';
import { ActionIcon, Tooltip } from '@mantine/core';
import { wsConnectionStatusAtom } from '../../atoms.ts';

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
        style={{ cursor: 'default' }}
      />
    </Tooltip>
  );
};

export default StatusIndicator;
