export default async function authFetch(
  url: string | URL,
  token: string,
  options: any = {},
) {
  // Use the user-provided options, but always override the authorization header with our own.
  const fullOptions = {
    ...options,
    headers: {
      ...options.headers,
      Authorization: `Bearer ${token}`,
    },
  };
  return fetch(url, fullOptions);
}
