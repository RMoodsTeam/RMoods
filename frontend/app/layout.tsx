import React from "react";
import type { Metadata } from "next";
import "./globals.css";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";
import Script from "next/script";
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
        <script id="theme">{`
      const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");
      darkThemeMq.addListener(e => {
      if (e.matches) {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
        }
      });
      if (localStorage.getItem("theme") === "light") {
        document.documentElement.classList.remove("dark");
      } else if (localStorage.getItem("theme") === "dark") {
        document.documentElement.classList.add("dark");
      } else {
        if (darkThemeMq.matches) {
          document.documentElement.classList.add("dark");
        }
        else {
          document.documentElement.classList.remove("dark");
        }
      }
      `}</script>
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
