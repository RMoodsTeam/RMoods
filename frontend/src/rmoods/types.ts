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
  analyses: ReportAnalysesMap;
  data_request: ReportDataRequest;
  created_at: string;
  updated_at: string;
}

export interface ReportQuery {
  userNamePattern?: string;
  containedAnalysisKinds: NlpAnalysisKind[];
  startDate?: string;
  endDate?: string;
  titlePattern?: string;
  includeMyReports: boolean;
  pagination: DbPagination;
}

export interface DbPagination {
  page: number;
  perPage: number;
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
}

export enum ReportStatusKind {
  Success = 'Success',
  InProgress = 'InProgress',
  Error = 'Error',
}

export interface ReportStatus {
  kind: ReportStatusKind;
  message?: string;
}

export interface NlpAnalysis {
  kind: NlpAnalysisKind;
  generatedIn: string;
  result: any;
}

export interface ReportAnalysesMap {
  nlpAnalysisKind: NlpAnalysisKind;
  nlpAnalysis: NlpAnalysis;
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
