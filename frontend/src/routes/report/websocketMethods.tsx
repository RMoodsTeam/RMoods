import { ReportResponse } from './types.ts';
import { notifications } from '@mantine/notifications';

/**
 * Handles incoming WebSocket messages and displays notifications based on the message status.
 *
 * @param {MessageEvent} event - The WebSocket message event.
 */
export const handleMessage = (event: MessageEvent) => {
  const response: ReportResponse = JSON.parse(event.data);

  if (response.status === 'ReportDone') {
    notifications.show({
      title: 'Report Done',
      message:
        'The report is ready to be viewed! Go to the reports page to see it.',
      color: 'blue',
      icon: '',
    });
  } else if (response.status === 'ReportError') {
    notifications.show({
      title: 'Error while requesting error',
      message: `An error occurred while requesting the report: "${response.data.message}"`,
      color: 'blue',
      icon: '',
    });
  }
};
