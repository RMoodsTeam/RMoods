import Cookies from 'js-cookie';
import { useAtom } from 'jotai';
import { notifications } from '@mantine/notifications';
import { useEffect, useRef } from 'react';
import { wsConnectionStatusAtom } from '../atoms.ts';

const WebsocketProvider = ({ children }: { children: React.ReactNode }) => {
  console.log('WebsocketProvider: rendering');

  const connection = useRef<WebSocket | null>(null);
  const [, setWsConnectionStatus] = useAtom(wsConnectionStatusAtom);

  useEffect(() => {
    const ws = new WebSocket(
      `ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get('RMOODS_JWT')}`
    );

    ws.onmessage = (event) => {
      console.log('Received WebSocket message');
      console.log(JSON.stringify(event.data));
      notifications.show({
        title: 'WebSocket Message',
        message:
          'Received a message from the WebSocket connection. Logged in console',
        color: 'blue',
        icon: '',
      });
    };

    ws.onopen = () => {
      console.log('WebSocket connection opened.');
      setWsConnectionStatus(true);
    };

    ws.onerror = (event) => {
      console.error(event);
      notifications.show({
        title: 'WebSocket Error',
        message: 'An error occurred with the WebSocket connection.',
        color: 'red',
        icon: '',
      });
      setWsConnectionStatus(false);
    };

    ws.onclose = () => {
      setWsConnectionStatus(false);
    };

    connection.current = ws;
    return () => {
      ws.close();
    };
  }, []);

  return <>{children}</>;
};

export default WebsocketProvider;

// SN: in case of problems with connection maybe use useEffect for this one
//
// MM: This sucks. React calls useEffect twice due to Strict Mode.
// We have to determine if that's something that we actually want to use or
// for now we're fine with double rendering and effect running.
// I'll leave it as it is for now, seems to be working somehow.
//
//   useEffect(() => {
//     if (webSocketConnection == null) {
//       webSocketConnection = new WebSocketConnection(setWsConnectionStatus);
//     }
//   }, [setWsConnectionStatus]);
//   return <>{children}</>;
// };

// export default WebsocketProvider;
