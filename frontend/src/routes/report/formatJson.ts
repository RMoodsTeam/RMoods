import { formValues } from './page.tsx';
import {
  AnalysisTypeSchema,
  DataSourceSchema,
  FeedKindSchema,
  FeedSortingSchema,
} from '../../rmoods/client/types.ts';
import { z } from 'zod';

const formattedJsonSchema = z.object({
  name: z.string(),
  resourceType: FeedKindSchema,
  isPublic: z.boolean(),
  size: z.string(),
  customSize: z.number().optional(),
  sorting: FeedSortingSchema,
  dataSource: z.array(DataSourceSchema),
  analyses: AnalysisTypeSchema,
});

type formattedJson = z.infer<typeof formattedJsonSchema>;

export const formatJson = (oldJson: formValues): formattedJson => {
  oldJson.dataSource = oldJson.dataSource.map((source: any) => {
    source.postId === '' ? (source.postId = null) : source.postId;
    return source;
  });

  // this variable will stay for a while as it help with debugging the new format
  const newJson = {
    name: oldJson.name,
    resourceType: oldJson.resourceType,
    isPublic: oldJson.isPublic === 'true',
    size: oldJson.size,
    customSize: oldJson.customSize,
    sorting: {
      kind: oldJson.sortBy,
      time: oldJson.time,
    },
    dataSource: oldJson.dataSource,
    analyses: oldJson.analyses,
  };
  return newJson;
};
