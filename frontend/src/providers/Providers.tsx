import React, { StrictMode } from 'react';
import { Provider as JotaiProvider, useAtomValue } from 'jotai';
import { GoogleOAuthProvider } from '@react-oauth/google';
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
            <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
              {children}
            </GoogleOAuthProvider>
          </MantineProvider>
        </QueryProvider>
      </JotaiProvider>
    </StrictMode>
  );
};

export default Providers;
