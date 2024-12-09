import { z } from 'zod';

export const FeedKindSchema = z.enum([
  'subredditPosts',
  'userPosts',
  'postComments',
]);

export const AnalysisTypeSchema = z.object({
  language: z.boolean(),
  sentiment: z.boolean(),
  sarcasm: z.boolean(),
  spam: z.boolean(),
  politics: z.boolean(),
  hateSpeech: z.boolean(),
  clickbait: z.boolean(),
  trolling: z.boolean(),
});

export const FeedSortingKindSchema = z.enum([
  'hot',
  'new',
  'rising',
  'top',
  'controversial',
]);

export const FeedSortingTimeSchema = z.enum([
  'day',
  'week',
  'month',
  'year',
  'all',
]);

export const FeedSortingSchema = z
  .object({
    kind: FeedSortingKindSchema,
    time: FeedSortingTimeSchema,
  })
  .refine((value) => {
    if (value.kind === 'top' || value.kind === 'controversial') {
      return value.time !== undefined;
    }
    return true;
  });

export const DataSourceSchema = z.object({
  name: z.string(),
  postId: z.string().optional(),
  share: z.number(),
});

const GoogleUserInfoSchema = z.object({
  sub: z.string(),
  name: z.string(),
  givenName: z.string(),
  familyName: z.string().nullable(),
  picture: z.string().url(),
  email: z.string().email(),
  emailVerified: z.boolean(),
});

const LanguageResponseSchema = z.object({
  language: z.array(z.string()),
  predicted: z.array(z.number()),
});

const NlpMetadataSchema = z.object({
  generatedIn: z.number(),
});

const NlpAnalysisSchema = z.object({
  metadata: NlpMetadataSchema,
  results: z.array(z.any()),
});

const ReportMetadataSchema = z.object({
  createdAt: z.number(),
  userInfo: GoogleUserInfoSchema,
  isPublic: z.boolean(),
});

const ReportResponseSchema = z.object({
  metadata: ReportMetadataSchema,
  analysis: NlpAnalysisSchema,
});

// Infer standard TS types from Zod schemas
export type FeedKind = z.infer<typeof FeedKindSchema>;
export type AnalysisType = z.infer<typeof AnalysisTypeSchema>;
export type FeedSorting = z.infer<typeof FeedSortingSchema>;
export type FeedSortingTime = z.infer<typeof FeedSortingTimeSchema>;
export type DataSource = z.infer<typeof DataSourceSchema>;
export type GoogleUserInfo = z.infer<typeof GoogleUserInfoSchema>;
export type LanguageResponse = z.infer<typeof LanguageResponseSchema>;
export type NlpMetadata = z.infer<typeof NlpMetadataSchema>;
export type NlpAnalysis = z.infer<typeof NlpAnalysisSchema>;
export type ReportMetadata = z.infer<typeof ReportMetadataSchema>;
export type ReportResponse = z.infer<typeof ReportResponseSchema>;
