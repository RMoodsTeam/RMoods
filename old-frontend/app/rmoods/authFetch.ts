export default async function authFetch(
  url: string | URL,
  token: string,
  options: any = {},
) {
  // Use the user-provided options, but always override the authorization header with our own.
  // Opt into NextJS `fetch` caching behaviour by setting `revalidate` to 30 seconds.
  const fullOptions = {
    ...options,
    headers: {
      ...options.headers,
      Authorization: `Bearer ${token}`,
    },
    next: {
      revalidate: 30,
    },
  };
  return fetch(url, fullOptions);
}
