/**
 * User interface representing the user data.
 */
export interface User {
  name: string;
  given_name: string;
  email: string;
  picture: string;
}

export interface Report {
  id: string;
  user_id: string;
  title: string;
  description: string;
  is_public: boolean;
  status: {
    Error?: string;
  };
  analyses: Record<string, unknown>;
  created_at: string;
  updated_at: string;
}
