import {
  FeedSorting,
  ReportFormAdaptedValues,
  ReportFormValues,
} from './schema.ts';
import { DataSource } from './schema.ts';

/**
 * Transforms the old JSON format from form to the new JSON format that backend expects.
 *
 * @param {ReportFormValues} oldJson - The old JSON format.
 * @returns {ReportFormAdaptedValues} - The new JSON format.
 */
export const transformJson = (
  oldJson: ReportFormValues
): ReportFormAdaptedValues => {
  // this variable will stay for a while as it help with debugging the new format
  const newJson = {
    name: oldJson.name,
    resourceKind: oldJson.resourceKind,
    isPublic: oldJson.isPublic === 'true',
    size: Number.parseInt(oldJson.size),
    sorting:
      oldJson.sortBy === 'top' || oldJson.sortBy === 'controversial'
        ? ({
            kind: oldJson.sortBy,
            time: oldJson.time,
          } as FeedSorting)
        : ({
            kind: oldJson.sortBy,
          } as FeedSorting),
    dataSources: oldJson.dataSources.map((source: DataSource) => {
      if (source.postId === '') {
        source.postId = undefined;
      }
      return source;
    }),
    analyses: Object.entries(oldJson.analyses)
      .filter(([_, value]) => value === true)
      .map(([key]) => String(key)),
  };
  return newJson;
};
