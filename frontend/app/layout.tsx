import React from "react";
import type { Metadata } from "next";
import "./globals.css";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";
import Providers from "./Providers";
import MainContainer from "./components/MainContainer";

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
    <html lang="en" suppressHydrationWarning>
      <head>
        {/*eslint-disable*/}
        <script id="theme" src="/initializeTheme.js" />
      </head>
      <body className=" bg-primary-light dark:bg-primary-dark text-primary-dark dark:text-primary-light transition-colors">
        <Providers>
          <Navbar />
          <MainContainer>{children}</MainContainer>
          <Footer />
        </Providers>
      </body>
    </html>
  );
}
