"use client";
import React from "react";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { Provider as JotaiProvider } from "jotai";
import ThemeRegistry from "./ThemeRegistry";

export default function Providers({ children }: { children: React.ReactNode }) {
  return (
    <ThemeRegistry options={{ key: "joy" }}>
      <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
        <JotaiProvider>{children}</JotaiProvider>
      </GoogleOAuthProvider>
    </ThemeRegistry>
  );
}
