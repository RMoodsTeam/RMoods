import Cookies from "js-cookie";
import {atom} from "jotai";
import {notifications} from "@mantine/notifications";

// Define an atom to store the WebSocket connection status
// for now the atom manipulation is in the onerror function as it was the easiest to test on
// later we can think about mocking the websocket connection but right now I'd focus on changing chakra version
// because more and more components will demand bigger refactor as we develop frontend
export const wsConnectionStatusAtom = atom<boolean>(false);

class WebSocketConnection {
  constructor(setWsConnectionStatus: (status: boolean) => void) {
    const wsClient = new WebSocket(`ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get("RMOODS_JWT")}`);

    wsClient.onmessage = (event) => {
      console.log("Received WebSocket message");
      console.log(JSON.stringify(event.data));
    };

    wsClient.onopen = () => {
      console.log("WebSocket connection opened.");
      setWsConnectionStatus(true); // Set the atom to true
    };

    wsClient.onerror = (event) => {
      console.error(event);

      notifications.show({
        title: 'WebSocket Error',
        message: "An error occurred with the WebSocket connection.",
        color: 'red',
        icon: 'bell',
      })

      setWsConnectionStatus(false); // Set the atom to false
      console.log("WebSocket connection status:", false);
    };

    wsClient.onclose = () => {
      console.log("WebSocket connection closed.");
    };
  }
}

export default WebSocketConnection;