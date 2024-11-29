import { formValues, transformedJson } from './types.ts';

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
    analyses: Object.entries(oldJson.analyses)
      .filter(([_, value]) => value === true)
      .map(([key]) => String(key)),
  };
  return newJson;
};
