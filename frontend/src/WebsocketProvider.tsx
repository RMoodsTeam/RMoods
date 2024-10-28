import WebSocketConnection from "./rmoods/websockets/wsClient.tsx";
import {useEffect} from "react";

const WebsocketProvider = ({children}: { children: React.ReactNode }) => {
  let webSocketConnection = null;

  //in case of problems with connection maybe use useEffect for this one
  useEffect(() => {
    if (webSocketConnection == null) {
      webSocketConnection = new WebSocketConnection();
    }
  }, [])
   return (
    <>
      {children}
    </>
  )
};

export default WebsocketProvider;