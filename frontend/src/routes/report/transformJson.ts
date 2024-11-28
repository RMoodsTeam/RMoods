import { formValues } from './page.tsx';
import {
  AnalysisTypeSchema,
  DataSourceSchema,
  FeedKindSchema,
  FeedSortingSchema,
} from '../../rmoods/client/types.ts';
import { z } from 'zod';

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
  analyses: AnalysisTypeSchema,
});

type transformedJson = z.infer<typeof transformedJsonSchema>;

/**
 * Transforms the old JSON format from form to the new JSON format that backend expects.
 *
 * @param {formValues} oldJson - The old JSON format.
 * @returns {transformedJson} - The new JSON format.
 */
export const transformJson = (oldJson: formValues): transformedJson => {
  // this variable will stay for a while as it help with debugging the new format
  const newJson = {
    name: oldJson.name,
    resourceType: oldJson.resourceType,
    isPublic: oldJson.isPublic === 'true',
    size: {
      kind: oldJson.size,
      customSize: oldJson.customSize,
    },
    customSize: oldJson.customSize,
    sorting: {
      kind: oldJson.sortBy,
      time: oldJson.time,
    },
    dataSource: oldJson.dataSource.map((source: any) => {
      source.postId === '' ? (source.postId = null) : source.postId;
      return source;
    }),
    analyses: oldJson.analyses,
  };
  return newJson;
};
