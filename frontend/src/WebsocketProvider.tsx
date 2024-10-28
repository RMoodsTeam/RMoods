import WebSocketConnection from "./rmoods/websockets/wsClient.tsx";

const WebsocketProvider = ({children}: { children: React.ReactNode }) => {

  //in case of problems with connection maybe use useEffect for this one
  const webSocketConnection = new WebSocketConnection();
   return (
    <>
      {children}
    </>
  )
};

export default WebsocketProvider;