import { z } from 'zod';
import {
  AnalysisTypeSchema,
  DataSourceSchema,
  FeedKindSchema,
  FeedSortingKindSchema,
  FeedSortingSchema,
  FeedSortingTimeSchema,
} from '../../rmoods/client/types.ts';

const ReportFormAdaptedSchema = z.object({
  name: z.string(),
  resourceType: FeedKindSchema,
  isPublic: z.boolean(),
  size: z.number().min(1).max(500),
  sorting: FeedSortingSchema,
  dataSource: z.array(DataSourceSchema),
  analyses: z.array(z.string()),
});

export const RowWrapperSchema = z.object({
  dataSource: DataSourceSchema,
  id: z.number(),
});

export const ReportFormValidationSchema = z.object({
  name: z.string().min(1, { message: 'Name must be longer than 1 character' }),
  resourceType: FeedKindSchema,
  isPublic: z.enum(['true', 'false']),
  size: z.string(), // string due to form api constraints
  sortBy: FeedSortingKindSchema,
  time: FeedSortingTimeSchema,
  dataSource: z
    .array(DataSourceSchema)
    .min(1, { message: 'At least 1 data source is required' }),
  analyses: AnalysisTypeSchema,
});

export type ReportFormValues = z.infer<typeof ReportFormValidationSchema>;
export type RowWrapper = z.infer<typeof RowWrapperSchema>;
export type ReportFormAdaptedValues = z.infer<typeof ReportFormAdaptedSchema>;
