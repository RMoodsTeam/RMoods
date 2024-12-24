import Cookies from 'js-cookie';
import { useAtom } from 'jotai';
import { useEffect, useRef } from 'react';
import { wsConnectionStatusAtom } from '../atoms.ts';
import { handleMessage } from '../routes/report/websocketMethods.tsx';
import BACKEND_URL from '../constants/backendUrl.ts';

const WebsocketProvider = ({ children }: { children: React.ReactNode }) => {
  const connection = useRef<WebSocket | null>(null);
  const [, setWsConnectionStatus] = useAtom(wsConnectionStatusAtom);
  useEffect(() => {
    const ws = new WebSocket(
      // .slice(7) removes the `https://` from the URL to make it work with the WebSocket
      `ws://${BACKEND_URL.slice(7)}/ws/connect?RMOODS_JWT=${Cookies.get('RMOODS_JWT')}`
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
