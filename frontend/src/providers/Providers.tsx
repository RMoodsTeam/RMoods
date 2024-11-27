import React, { StrictMode } from 'react';
import { Provider as JotaiProvider, useAtomValue } from 'jotai';
import { GoogleOAuthProvider } from '@react-oauth/google';
import WebsocketProvider from './WebsocketProvider.tsx';
import { MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { colorModeAtom } from '../atoms.ts';
import QueryProvider from './QueryProvider.tsx';

const Providers = ({ children }: { children: React.ReactNode }) => {
  const colorScheme = useAtomValue(colorModeAtom);

  return (
    <StrictMode>
      <JotaiProvider>
        <QueryProvider>
          <MantineProvider defaultColorScheme={colorScheme}>
            <Notifications />
            <WebsocketProvider>
              <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
                {children}
              </GoogleOAuthProvider>
            </WebsocketProvider>
          </MantineProvider>
        </QueryProvider>
      </JotaiProvider>
    </StrictMode>
  );
};

export default Providers;
