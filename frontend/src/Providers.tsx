import { StrictMode } from "react";
import { ChakraProvider } from "@chakra-ui/react";
import { Provider as JotaiProvider } from "jotai";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { rmoodsTheme } from "./theme.ts";
import WebsocketProvider from "./WebsocketProvider.tsx";

const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <StrictMode>
      <ChakraProvider theme={rmoodsTheme} resetCSS>
      <WebsocketProvider>
      <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
          <JotaiProvider>{children}</JotaiProvider>
      </GoogleOAuthProvider>
      </WebsocketProvider>
      </ChakraProvider>
    </StrictMode>
  );
};

export default Providers;
