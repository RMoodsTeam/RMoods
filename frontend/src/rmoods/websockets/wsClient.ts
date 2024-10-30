import Cookies from "js-cookie";
import {CreateToastFnReturn} from "@chakra-ui/react";
import {atom} from "jotai";

// Define an atom to store the WebSocket connection status
export const wsConnectionStatusAtom = atom<boolean>(false);

class WebSocketConnection {
  constructor(toast: CreateToastFnReturn, setWsConnectionStatus: (status: boolean) => void) {
    const wsClient = new WebSocket(`ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get("RMOODS_JWT")}`);

    wsClient.onmessage = (event) => {
      console.log(JSON.stringify(event.data));
    };

    wsClient.onopen = () => {
      console.log("Hello on open!");
      setWsConnectionStatus(true); // Set the atom to true
      console.log("WebSocket connection status:", true);
    };

    wsClient.onerror = (event) => {
      console.error(event);

      toast({
        title: 'WebSocket Error',
        description: "An error occurred with the WebSocket connection.",
        status: 'error',
        duration: 5000,
        isClosable: true,
      });

      setWsConnectionStatus(false); // Set the atom to false
      console.log("WebSocket connection status:", false);
    };

    wsClient.onclose = () => {
      console.log("Bye!");
    };
  }
}

export default WebSocketConnection;