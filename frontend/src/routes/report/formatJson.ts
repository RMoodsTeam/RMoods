import { formValues } from './page.tsx';

export const formatJson = (oldJson: any) => {
  oldJson.dataSource = oldJson.dataSource.map((source: any) => {
    source.postId === '' ? (source.postId = null) : source.postId;
    return source;
  });
  // this variable will stay for a while as it help with debugging the new format
  const newJson = {
    name: oldJson.name,
    resourceType: oldJson.resourceType,
    isPublic: oldJson.isPublic,
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
