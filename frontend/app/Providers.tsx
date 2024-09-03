"use client";
import React from "react";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { Provider as JotaiProvider } from "jotai";
import { AppRouterCacheProvider } from "@mui/material-nextjs/v14-appRouter";
import theme from "./theme";
import { ThemeProvider } from "@mui/material";

export default function Providers({ children }: { children: React.ReactNode }) {
  return (
    <AppRouterCacheProvider options={{ enableCssLayer: true }}>
      <ThemeProvider theme={theme}>
        <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
          <JotaiProvider>{children}</JotaiProvider>
        </GoogleOAuthProvider>
      </ThemeProvider>
    </AppRouterCacheProvider>
  );
}
