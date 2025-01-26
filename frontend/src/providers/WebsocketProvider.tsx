import Cookies from 'js-cookie';
import { useAtom } from 'jotai';
import { useEffect } from 'react';
import { wsConnectionStatusAtom } from '../atoms.ts';
import { handleMessage } from '../routes/report/websocketMethods.tsx';
import BACKEND_URL from '../constants/backendUrl.ts';
import useWebSocket from 'react-use-websocket';
import { notifications } from '@mantine/notifications';

/**
 * This component establishes a WebSocket connection to the server and provides
 * WebSocket functionality to its child components.
 * 
 * @param {Object} props - The component props
 * @param {React.ReactNode} props.children - The child components to render within the provider
 * @returns {JSX.Element} The WebsocketProvider component.
 */

const WebsocketProvider = ({ children }: { children: React.ReactNode }) => {
  const [, setWsConnectionStatus] = useAtom(wsConnectionStatusAtom);
  const { sendMessage, lastMessage, readyState } = useWebSocket(
    `ws://${BACKEND_URL.slice(7)}/ws/connect?RMOODS_JWT=${Cookies.get('RMOODS_JWT')}`,
    {
      onOpen: () => {
        console.log('WebSocket connection opened.');
        setWsConnectionStatus(true);
      },
      onClose: () => {
        console.log('WebSocket connection closed.');
        setWsConnectionStatus(false);
      },
      onError: (event) => {
        console.error('WebSocket error:.', event);
        // WARNING: this is only commented out for development purposes,
        // this should be uncommented when deploying
        // notifications.show({
        //   title: 'WebSocket Error',
        //   message: 'An error occurred with the WebSocket connection.',
        //   color: 'red',
        //   icon: '',
        // });
        setWsConnectionStatus(false);
      },
      shouldReconnect: () => true,
    }
  );

  useEffect(() => {
    if (lastMessage !== null) {
      handleMessage(lastMessage);
    }
  }, [lastMessage]);

  return <>{children}</>;
};

export default WebsocketProvider;
