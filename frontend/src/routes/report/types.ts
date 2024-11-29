import { z } from 'zod';
import {
  AnalysisTypeSchema,
  DataSourceSchema,
  FeedKindSchema,
  FeedSortingKindSchema,
  FeedSortingSchema,
  FeedSortingTimeSchema,
} from '../../rmoods/client/types.ts';

const transformedJsonSchema = z.object({
  name: z.string(),
  resourceType: FeedKindSchema,
  isPublic: z.boolean(),
  size: z.object({
    kind: z.string(),
    customSize: z.number().optional(),
  }),
  customSize: z.number().optional(),
  sorting: FeedSortingSchema,
  dataSource: z.array(DataSourceSchema),
  analyses: z.array(z.string()),
});

export const RowWrapperSchema = z.object({
  dataSource: DataSourceSchema,
  id: z.number(),
});

export const formValidationSchema = z.object({
  name: z.string().min(1, { message: 'Name must be longer than 1 character' }),
  resourceType: FeedKindSchema,
  isPublic: z.enum(['true', 'false']),
  size: z.enum(['small', 'medium', 'large', 'custom']),
  customSize: z.number().optional(),
  sortBy: FeedSortingKindSchema,
  time: FeedSortingTimeSchema,
  dataSource: z
    .array(DataSourceSchema)
    .min(1, { message: 'At least 1 data source is required' }),
  analyses: AnalysisTypeSchema,
});

export type formValues = z.infer<typeof formValidationSchema>;
export type RowWrapper = z.infer<typeof RowWrapperSchema>;
export type transformedJson = z.infer<typeof transformedJsonSchema>;
