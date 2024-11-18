import {z} from 'zod';

const FeedKindSchema = z.enum(['subredditPosts', 'userPosts', 'postComments']);
const AnalysisTypeSchema = z.enum(['language']);
const FeedSortingKindSchema = z.enum(['hot', 'new', 'rising', 'top', 'controversial']);
const FeedSortingTimeSchema = z.enum(['day', 'week', 'month', 'year', 'all']);

const FeedSortingSchema = z.object({
  kind: FeedSortingKindSchema,
  time: FeedSortingTimeSchema
});

const DataSourceSchema = z.object({
  name: z.string(),
  post_id: z.string().optional(),
  share: z.number().min(0).max(100)
});

const FeedRequestSchema = z.object({
  resourceKind: FeedKindSchema,
  reportTypes: z.array(AnalysisTypeSchema),
  dataSources: z.array(DataSourceSchema),
  size: z.number().min(1),
  sorting: FeedSortingSchema
});

const GoogleUserInfoSchema = z.object({
  sub: z.string(),
  name: z.string(),
  given_name: z.string(),
  family_name: z.string().nullable(),
  picture: z.string().url(),
  email: z.string().email(),
  email_verified: z.boolean()
})

const LanguageResponseSchema = z.object({
  language: z.array(z.string()),
  predicted: z.array(z.number())
});

const NlpMetadataSchema = z.object({
  generated_in: z.number(),
});

const NlpAnalysisSchema = z.object({
  metadata: NlpMetadataSchema,
  results: z.array(z.any())
})

const ReportMetadataSchema = z.object({
  created_at: z.number(),
  user_info: GoogleUserInfoSchema,
  is_public: z.boolean()
})

const ReportResponseSchema = z.object({
  metadata: ReportMetadataSchema,
  analysis: NlpAnalysisSchema
})

// Infer standard TS types from Zod schemas
export type FeedKind = z.infer<typeof FeedKindSchema>;
export type AnalysisType = z.infer<typeof AnalysisTypeSchema>;
export type FeedSorting = z.infer<typeof FeedSortingSchema>;
export type FeedSortingTime = z.infer<typeof FeedSortingTimeSchema>;
export type DataSource = z.infer<typeof DataSourceSchema>;
export type FeedRequest = z.infer<typeof FeedRequestSchema>;
export type GoogleUserInfo = z.infer<typeof GoogleUserInfoSchema>;
export type LanguageResponse = z.infer<typeof LanguageResponseSchema>;
export type NlpMetadata = z.infer<typeof NlpMetadataSchema>;
export type NlpAnalysis = z.infer<typeof NlpAnalysisSchema>;
export type ReportMetadata = z.infer<typeof ReportMetadataSchema>;
export type ReportResponse = z.infer<typeof ReportResponseSchema>;