import { z } from 'zod';
import {
  AnalysisTypeSchema,
  DataSourceSchema,
  FeedKindSchema,
  FeedSortingKindSchema,
  FeedSortingSchema,
  FeedSortingTimeSchema,
} from '../../rmoods/client/types.ts';
import { UseFormReturnType } from '@mantine/form';

const ReportFormAdaptedSchema = z.object({
  name: z.string(),
  resourceKind: FeedKindSchema,
  isPublic: z.boolean(),
  size: z.number().min(1).max(500),
  sorting: FeedSortingSchema,
  dataSources: z.array(DataSourceSchema),
  analyses: z.array(z.string()),
});

export const RowWrapperSchema = z.object({
  dataSource: DataSourceSchema,
  id: z.number(),
});

export const FetchSizeSchema = z.enum(['30', '70', '100', 'custom']);

export const ReportFormValidationSchema = z.object({
  name: z.string().min(1, { message: 'Name must be longer than 1 character' }),
  resourceKind: FeedKindSchema,
  isPublic: z.enum(['true', 'false']),
  size: FetchSizeSchema, // string due to form api constraints
  sortBy: FeedSortingKindSchema,
  time: FeedSortingTimeSchema,
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
export type ReportFormValues = z.infer<typeof ReportFormValidationSchema>;
export type RowWrapper = z.infer<typeof RowWrapperSchema>;
export type ReportFormAdaptedValues = z.infer<typeof ReportFormAdaptedSchema>;
