import React from "react";
import type { Metadata } from "next";
import "./globals.css";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";
import Providers from "./Providers";
import MainContainer from "./components/MainContainer";
import InitColorSchemeScript from "@mui/joy/InitColorSchemeScript";

export const metadata: Metadata = {
  title: "RMoods",
  description: "AI-powered Redditor!",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      suppressHydrationWarning
      style={{ backgroundColor: "var(--joy-palette-background-body)" }}
    >
      <head>
        <meta name="viewport" content="initial-scale=1, width=device-width" />
        <InitColorSchemeScript />
      </head>
      <Providers>
        <body
          style={{
            minHeight: "100vh",
            display: "grid",
            gridTemplateRows: "auto 1fr auto",
          }}
        >
          <Navbar />
          <MainContainer>{children}</MainContainer>
          <Footer />
        </body>
      </Providers>
    </html>
  );
}
