import Cookies from "js-cookie";

class WebSocketConnection {
  constructor() {
    const wsClient = new WebSocket(`ws://localhost:8001/ws/connect?${Cookies.get("RMOODS_JWT")}`);
    wsClient.onmessage = (event) => {
      console.log(JSON.stringify(event.data));
    }
    wsClient.onopen = () => {
      console.log("Hello!");
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