import { z } from 'zod';
import { UseFormReturnType } from '@mantine/form';

export const DataSourceSchema = z.object({
  name: z.string(),
  postId: z.string().optional(),
  share: z.number(),
});

export const FeedKindSchema = z.enum([
  'subredditPosts',
  'userPosts',
  'postComments',
]);

export const AnalysisTypeSchema = z
  .object({
    language: z.boolean(),
    sentiment: z.boolean(),
    sarcasm: z.boolean(),
    spam: z.boolean(),
    politics: z.boolean(),
    hateSpeech: z.boolean(),
    clickbait: z.boolean(),
    trolling: z.boolean(),
  })
  .refine(
    (value) => {
      return Object.values(value).some((val) => val === true);
    },
    { message: 'At least 1 analysis type is required' }
  );

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

const ReportFormAdaptedSchema = z.object({
  title: z.string(),
  description: z.string().max(500),
  isPublic: z.boolean(),
  dataRequest: z.object({
    resourceKind: FeedKindSchema,
    size: z.number().min(1).max(500),
    sortBy: FeedSortingSchema,
    dataSources: z.array(DataSourceSchema).min(1),
  }),
  nlpRequest: z.object({
    analyses: z.array(z.string()),
  }),
});

export const RowWrapperSchema = z.object({
  dataSource: DataSourceSchema,
  id: z.number(),
});

export const FetchSizeSchema = z.string();

/*
 * Flattened schema for the form values, later converted to the backend format.
 */
export const ReportFormValuesSchema = z.object({
  title: z.string().min(1, { message: 'Title cannot be empty' }),
  description: z.string().max(500, {
    message: 'Description must be shorter than 500 characters',
  }),
  isPublic: z.enum(['true', 'false']),
  resourceKind: FeedKindSchema,
  size: FetchSizeSchema, // string due to form api constraints
  sortBy: FeedSortingKindSchema,
  time: FeedSortingTimeSchema.nullable(),
  dataSources: z
    .array(DataSourceSchema)
    .min(1, { message: 'At least 1 data source is required' }),
  analyses: AnalysisTypeSchema,
});

const ReportResponseSchema = z.object({
  status: z.enum(['ReportDone', 'ReportError']),
  data: z.object({ code: z.number(), message: z.string() }),
});

export type MantineReportForm = UseFormReturnType<ReportFormValues>;
export type ReportResponse = z.infer<typeof ReportResponseSchema>;
export type ReportFormValues = z.infer<typeof ReportFormValuesSchema>;
export type RowWrapper = z.infer<typeof RowWrapperSchema>;
export type ReportFormAdaptedValues = z.infer<typeof ReportFormAdaptedSchema>;
export type FeedKind = z.infer<typeof FeedKindSchema>;
export type AnalysisType = z.infer<typeof AnalysisTypeSchema>;
export type FeedSorting = z.infer<typeof FeedSortingSchema>;
export type FeedSortingTime = z.infer<typeof FeedSortingTimeSchema>;
export type DataSource = z.infer<typeof DataSourceSchema>;
