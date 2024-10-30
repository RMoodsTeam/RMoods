import Cookies from "js-cookie";
import {CreateToastFnReturn, useToast} from "@chakra-ui/react";

class WebSocketConnection {
  constructor(toast: CreateToastFnReturn) {
    const wsClient = new WebSocket(`ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get("RMOODS_JWT")}`);

    wsClient.onmessage = (event) => {
      console.log(JSON.stringify(event.data));
    }
    wsClient.onopen = () => {
      console.log("Hello on open!");
    }
    wsClient.onerror = (event) => {
      console.error(event);

      toast({
        title: 'WebSocket Error',
        description: "An error occurred with the WebSocket connection.",
        status: 'error',
        duration: 5000,
        isClosable: true,
      });
    }
    wsClient.onclose = () => {
      console.log("Bye!");
    }
  }
}

export default WebSocketConnection;