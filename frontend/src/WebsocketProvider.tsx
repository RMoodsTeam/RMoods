import WebSocketConnection, {wsConnectionStatusAtom} from "./rmoods/websockets/wsClient.ts";
import {useEffect} from "react";
import {useSetAtom} from "jotai";

const WebsocketProvider = ({children}: { children: React.ReactNode }) => {
  let webSocketConnection: WebSocketConnection | null = null;
  const setWsConnectionStatus = useSetAtom(wsConnectionStatusAtom);

  // SN: in case of problems with connection maybe use useEffect for this one
  //
  // MM: This sucks. React calls useEffect twice due to Strict Mode.
  // We have to determine if that's something that we actually want to use or
  // for now we're fine with double rendering and effect running.
  // I'll leave it as it is for now, seems to be working somehow.
  useEffect(() => {
    if (webSocketConnection == null) {
      webSocketConnection = new WebSocketConnection(setWsConnectionStatus);
    }
  }, [setWsConnectionStatus])
  return (
    <>
      {children}
    </>
  )
};

export default WebsocketProvider;