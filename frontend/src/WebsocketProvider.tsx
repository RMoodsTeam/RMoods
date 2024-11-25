import { useEffect } from 'react';
import Cookies from 'js-cookie';
import { useAtom, useSetAtom } from 'jotai';
import { atom } from 'jotai';
import { notifications } from '@mantine/notifications';

export const wsConnectionStatusAtom = atom<boolean>(false);

const WebsocketProvider = ({ children }: { children: React.ReactNode }) => {
  console.log('WebsocketProvider: rendering');
  const [wsConnection, setWsConnectionStatus] = useAtom(wsConnectionStatusAtom);
      const wsClient = new WebSocket(
        `ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get('RMOODS_JWT')}`
      );

  useEffect(() => {
    console.log('WebSocket connection status updated:', {wsConnection, timestamp: new Date().toISOString(),});
  }, [wsConnection]);

  useEffect(() => {

    wsClient.onmessage = (event) => {
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

    wsClient.onopen = () => {
      console.log('WebSocket connection opened.');
      setWsConnectionStatus(true);
      console.log('WebSocket connection status:' , wsConnection);
    };

    wsClient.onerror = (event) => {
      console.error(event);
      notifications.show({
        title: 'WebSocket Error',
        message: 'An error occurred with the WebSocket connection.',
        color: 'red',
        icon: '',
      });
      setWsConnectionStatus(false);
      console.log('WebSocket connection status:',  wsConnection);
    };

    wsClient.onclose = () => {
      console.log('WebSocket connection closed.');
      setWsConnectionStatus(false);
    };

    // return () => {
    //   wsClient.close();
    //   setWsConnectionStatus(false);
    // };
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
