import React from 'react';
import { useAtomValue } from 'jotai';
import { ActionIcon, Tooltip } from '@mantine/core';
import { wsConnectionStatusAtom } from '../../atoms.ts';

const StatusIndicator = () => {
  const isConnected = useAtomValue(wsConnectionStatusAtom);
  console.log('StatusIndicator: rendering');
  console.log('StatusIndicator: connection status:', isConnected);

  return (
    <Tooltip
      label={`WebSocket Status: ${isConnected} (${new Date().toLocaleTimeString()})`}
    >
      <ActionIcon
        size="xs"
        radius="100%"
        color={isConnected ? 'green' : 'red'}
        style={{
          border: '2px solid rgba(0, 0, 0, 0.5)',
          boxShadow: '0 2px 4px rgba(0, 0, 0, 0.2)',
        }}
        onClick={() => console.log('Current ws status:', isConnected)}
      />
    </Tooltip>
  );
};

export default StatusIndicator;
