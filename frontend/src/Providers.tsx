import React, {StrictMode} from "react";
import {Provider as JotaiProvider} from "jotai";
import {GoogleOAuthProvider} from "@react-oauth/google";
import WebsocketProvider from "./WebsocketProvider.tsx";
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import {MantineProvider} from '@mantine/core';
import {Notifications} from "@mantine/notifications";
import {useAtomValue} from "jotai/ts3.8/react/useAtomValue";
import {colorModeAtom} from "./atoms.ts";

const Providers = ({children}: { children: React.ReactNode }) => {
  const colorScheme = useAtomValue(colorModeAtom);
  
  return (
    <StrictMode>
      <MantineProvider defaultColorScheme={colorScheme}>
        <Notifications/>
        <WebsocketProvider>
          <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
            <JotaiProvider>{children}</JotaiProvider>
          </GoogleOAuthProvider>
        </WebsocketProvider>
      </MantineProvider>
    </StrictMode>
  );
};

export default Providers;
