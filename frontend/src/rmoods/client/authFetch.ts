import Cookies from 'js-cookie';
import BACKEND_URL from '../../constants/backendUrl.ts';

export default async function authFetch(
  url: string | URL,
  options: RequestInit = {}
) {
  const token = Cookies.get('RMOODS_JWT');

  // Use the user-provided options, but always override the authorization header with our own.
  const fullOptions = {
    ...options,
    headers: {
      ...options.headers,
      Authorization: `Bearer ${token}`,
    },
  };
  return fetch(BACKEND_URL + '/api' + url, fullOptions);
}

// TODO: Add unit tests for the options merging logic
