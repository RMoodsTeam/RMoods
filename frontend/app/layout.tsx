import React from "react";
import type { Metadata } from "next";
import "./globals.css";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";
import Script from "next/script";
import { GoogleOAuthProvider } from "@react-oauth/google";
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
    <html lang="en">
      <Script id="theme">
        {`  if (
              localStorage.theme === "dark" ||
              (!("theme" in localStorage) &&
                window.matchMedia("(prefers-color-scheme: dark)").matches)
            ) {
              document.documentElement.classList.add("dark");
            } else {
              document.documentElement.classList.remove("dark");
            }`}
      </Script>
      <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
        <body className=" bg-primary-light dark:bg-primary-dark text-primary-dark dark:text-primary-light">
          <Navbar />
          <MainContainer>{children}</MainContainer>
          <Footer />
        </body>
      </GoogleOAuthProvider>
    </html>
  );
}
