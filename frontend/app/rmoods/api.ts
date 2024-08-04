"use server";
// Just for now, those functions are in this single file.
// Once the amount grows, we want to move them to a separate directory

import { cookies } from "next/headers";
import authFetch from "./authFetch";

const EndpointMapper = {
  "user-info": (u: string) => "/api/debug/user-info?u=" + u,
  "subreddit-info": (r: string) => "/api/debug/subreddit-info?r=" + r,
};

async function fetchEndpoint(endpoint: string): Promise<object> {
  const API_URL = "http://localhost:8001";
  const token = cookies().get("RMOODS_JWT")?.value;
  if (!token) {
    throw new Error("No token found");
  }
  const response = await authFetch(API_URL + endpoint, token);
  return response.json();
}

export async function fetchUserInfo(username: string): Promise<object> {
  const endpoint = EndpointMapper["user-info"](username);
  return fetchEndpoint(endpoint);
}

export async function fetchSubredditInfo(subreddit: string): Promise<object> {
  const endpoint = EndpointMapper["subreddit-info"](subreddit);
  return fetchEndpoint(endpoint);
}
