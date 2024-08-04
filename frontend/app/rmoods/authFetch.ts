import { cookies } from "next/headers";

export default function rmoodsFetch(endpoint: string, options: any = {}) {
  const token = cookies().get("RMOODS_JWT");
  const url = "http://localhost:3001" + endpoint;
  const fullOptions = {
    ...options,
    headers: {
      ...options.headers,
      'Authorization': `Bearer ${token}`,
    }
  };
  return fetch(url, fullOptions);
}
