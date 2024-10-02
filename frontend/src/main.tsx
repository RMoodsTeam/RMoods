import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { ChakraProvider } from "@chakra-ui/react";
import { Provider as JotaiProvider } from "jotai";
import "./index.css";
import { RouterProvider } from "react-router-dom";
import router from "./router.tsx";
import { GoogleOAuthProvider } from "@react-oauth/google";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <GoogleOAuthProvider clientId="1055063718392-2ajj0s8h3pol9u5fdlt5vg8jep200r6i.apps.googleusercontent.com">
      <ChakraProvider>
        <JotaiProvider>
          <RouterProvider router={router} />
        </JotaiProvider>
      </ChakraProvider>
    </GoogleOAuthProvider>
  </StrictMode>
);
