import authFetch from './authFetch.ts';
import { ReportFormAdaptedValues } from '../../routes/report/schema.ts';
import { User } from '../types.ts';

export type ReportRequest = ReportFormAdaptedValues;

/**
 * Provides a simple interface to the RMoods Backend API.
 */
export class RMoodsClient {
  /**
   * Requests a report from the RMoods API
   *
   * Sends a POST request to the RMoods API to request a report
   * based on the given request object. The request object should be a valid
   * `FeedRequest` object as defined in the schema.ts file.
   *
   * The function will throw an error if the request fails for any reason.
   * We expect a `ReportAck` to come back from this request.
   * @param request
   */
  static async requestReport(request: ReportRequest): Promise<Response> {
    return await authFetch(`/report`, {
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

  static async getUserInfo(userId: string): Promise<User> {
    return await authFetch(`${RMoodsClient.URL}/user?id=${userId}`).then(
      (res) => {
        if (!res.ok) {
          throw new Error('Failed to fetch user data');
        }
        return res.json();
      }
    );
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
