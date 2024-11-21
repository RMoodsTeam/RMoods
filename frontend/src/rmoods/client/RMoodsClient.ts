import { FeedRequest } from './types.ts';
import authFetch from './authFetch.ts';
import Cookies from 'js-cookie';

/**
 * Provides a simple interface to the RMoods Backend API.
 */
export default class RMoodsClient {
  static URL = 'http://localhost:8001/api';

  /**
   * Requests a report from the RMoods API
   *
   * Sends a POST request to the RMoods API to request a report
   * based on the given request object. The request object should be a valid
   * `FeedRequest` object as defined in the types.ts file.
   *
   * The function will throw an error if the request fails for any reason.
   * We expect a `ReportAck` to come back from this request.
   * @param request
   */
  static async requestReport(request: FeedRequest): Promise<Response> {
    const jwt = Cookies.get('RMOODS_JWT');
    if (!jwt) {
      throw new Error('No JWT token found when trying to request a report');
    }

    return await authFetch(`${RMoodsClient.URL}/report`, jwt, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    }).then((res) => {
      console.log('Requested a report');
      console.log(res);
      if (!res.ok) {
        throw new Error('Failed to request report');
      }
      return res.json();
    });
  }

  /**
   * Fetch information about a subreddit using the `/about/subreddit` endpoint
   */
  static async fetchAboutSubreddit() {}

  /**
   * Fetch information about a user using the `/about/user` endpoint
   */
  static async fetchAboutUser() {}
}
