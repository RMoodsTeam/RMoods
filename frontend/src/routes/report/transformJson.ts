import { DataSource, FeedSorting, ReportFormAdaptedValues, ReportFormValues } from './schema.ts';

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
  const newJson: ReportFormAdaptedValues = {
    title: oldJson.title,
    description: oldJson.description,
    isPublic: oldJson.isPublic === 'true',
    dataRequest: {
      feedKind: oldJson.feedKind,
      size: Number.parseInt(oldJson.size),
      sortBy:
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
    },
    nlpRequest: {
      analyses: Object.entries(oldJson.analyses)
        .filter(([_, value]) => value === true)
        .map(([key]) => String(key)),
    },
  };
  console.log(newJson);
  return newJson;
};
