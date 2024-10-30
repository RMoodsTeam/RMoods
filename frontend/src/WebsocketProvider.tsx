import WebSocketConnection from "./rmoods/websockets/wsClient.ts";
import {useEffect} from "react";
import {useToast} from "@chakra-ui/react";

const WebsocketProvider = ({children}: { children: React.ReactNode }) => {
  let webSocketConnection : WebSocketConnection | null = null;
  const toast = useToast()

  // SN: in case of problems with connection maybe use useEffect for this one
  //
  // MM: This sucks. React calls useEffect twice due to Strict Mode.
  // We have to determine if that's something that we actually want to use or
  // for now we're fine with double rendering and effect running.
  // I'll leave it as it is for now, seems to be working somehow.
  useEffect(() => {
    if (webSocketConnection == null) {
      webSocketConnection = new WebSocketConnection(toast);
    }
  }, [toast])
  return (
    <>
      {children}
    </>
  )
};

export default WebsocketProvider;