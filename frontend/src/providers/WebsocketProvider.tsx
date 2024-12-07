import Cookies from 'js-cookie';
import { useAtom } from 'jotai';
import { useEffect, useRef } from 'react';
import { wsConnectionStatusAtom } from '../atoms.ts';
import { handleMessage } from '../routes/report/websocketMethods.tsx';

const WebsocketProvider = ({ children }: { children: React.ReactNode }) => {
  const connection = useRef<WebSocket | null>(null);
  const [, setWsConnectionStatus] = useAtom(wsConnectionStatusAtom);
  useEffect(() => {
    const ws = new WebSocket(
      `ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get('RMOODS_JWT')}`
    );

    ws.onmessage = handleMessage;

    ws.onopen = () => {
      console.log('WebSocket connection opened.');
      setWsConnectionStatus(true);
    };

    ws.onerror = (event) => {
      console.error(event);
      // WARNING: this is only commented out for development purposes,
      // this should be uncommented when deploying

      // notifications.show({
      //   title: 'WebSocket Error',
      //   message: 'An error occurred with the WebSocket connection.',
      //   color: 'red',
      //   icon: '',
      // });
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
