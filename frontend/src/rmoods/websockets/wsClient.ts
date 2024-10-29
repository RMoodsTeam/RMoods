import Cookies from "js-cookie";

class WebSocketConnection {
  constructor() {
    console.log("Hello!")
    const wsClient = new WebSocket(`ws://localhost:8001/ws/connect?RMOODS_JWT=${Cookies.get("RMOODS_JWT")}`);

    wsClient.onmessage = (event) => {
      console.log(JSON.stringify(event.data));
    }
    wsClient.onopen = () => {
      console.log("Hello on open!");
    }
    wsClient.onerror = (event) => {
      console.error(event);
    }
    wsClient.onclose = () => {
      console.log("Bye!");
    }
  }
}

export default WebSocketConnection;