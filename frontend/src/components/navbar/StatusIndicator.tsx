import React, { useEffect } from 'react';
import { useAtom } from 'jotai';
import { wsConnectionStatusAtom } from '../../WebsocketProvider';
import { ActionIcon, Tooltip } from '@mantine/core';

const StatusIndicator = () => {
  const [isConnected] = useAtom(wsConnectionStatusAtom);
  
  useEffect(() => {
    console.log('StatusIndicator: connection status changed:', {
      isConnected,
      timestamp: new Date().toISOString(),
    });
  }, [isConnected]);

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
