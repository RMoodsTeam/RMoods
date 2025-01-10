// @ts-ignore

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
  status: ReportStatus;
  analyses: Map<NlpAnalysisKind, NlpAnalysis>;
  data_request: ReportDataRequest;
  created_at: Date;
  updated_at: Date;
}

// Adapted to fit the Mantine inputs, so for example the date is a string
export type ReportQuery = {
  userNamePattern?: string;
  containedAnalysisKinds: NlpAnalysisKind[];
  startDate?: Date;
  endDate?: Date;
  titlePattern?: string;
  includeMyReports: boolean;
} & DbPagination;

export interface DbPagination {
  page: number;
  perPage: number;
}

/**
 * Response from /report endpoint for fetching reports by query
 */
export interface ReportQueryResponse {
  reports: Report[];
  totalPages: number;
}

export enum NlpAnalysisKind {
  Clickbait = 'clickbait',
  HateSpeech = 'hateSpeech',
  Keywords = 'keywords',
  Language = 'language',
  Politics = 'politics',
  Sarcasm = 'sarcasm',
  Sentiment = 'sentiment',
  Spam = 'spam',
  LLM = 'llm',
}

export class NlpAnalysisKindUtil {
  static getDisplayLabel(kind: NlpAnalysisKind): string {
    switch (kind) {
      case NlpAnalysisKind.Clickbait:
        return 'Clickbait';
      case NlpAnalysisKind.HateSpeech:
        return 'Hate Speech';
      case NlpAnalysisKind.Keywords:
        return 'Keywords';
      case NlpAnalysisKind.Language:
        return 'Language';
      case NlpAnalysisKind.Politics:
        return 'Politics';
      case NlpAnalysisKind.Sarcasm:
        return 'Sarcasm';
      case NlpAnalysisKind.Sentiment:
        return 'Sentiment';
      case NlpAnalysisKind.Spam:
        return 'Spam';
      case NlpAnalysisKind.LLM:
        return 'LLM Detection';
    }
  }
}

export enum ReportStatusKind {
  Success = 'Success',
  InProgress = 'InProgress',
  Error = 'Error',
}

export interface ReportStatus {
  status: ReportStatusKind;
  message?: string;
}

export interface NlpAnalysis {
  kind: NlpAnalysisKind;
  generatedIn: string;
  result: any;
}

interface DataSources {
  name: string;
  postId: string;
  sharer: number;
}

enum RedditFeedKind {
  UserPost = 'UserPost',
  PostComments = 'PostComments',
  SubredditPosts = 'SubredditPosts',
}

enum FeedSorting {
  Hot = 'Hot',
  New = 'New',
  Rising = 'Rising',
  Top = 'Top',
  Controversial = 'Controversial',
}

interface SortBy {
  kind: FeedSorting;
}

interface ReportDataRequest {
  feedKind: RedditFeedKind;
  dataSources: DataSources[];
  size: number;
  sortBy: SortBy;
}
