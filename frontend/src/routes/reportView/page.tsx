import { Accordion, Card, Flex, Stack } from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';
import styles from './page.module.scss';

const ReportElement = ({ value, description }) => {
  return (
    <Card className={styles.item}>
      <Accordion>
        <Accordion.Item key={value} value={value}>
          <Accordion.Control>{value}</Accordion.Control>
          <Accordion.Panel>{description}</Accordion.Panel>
        </Accordion.Item>
      </Accordion>
    </Card>
  );
};

const ReportView = () => {
  return (
    <Stack className={styles.wrapper}>
      <ReportElement value="item.value1" description="item.description" />
      <ReportElement value="item.value2" description="item.description" />
      <ReportElement value="item.value3" description="item.description" />
    </Stack>
  );
};

export default function () {
  return (
    <ErrorBoundary FallbackComponent={PageFallback}>
      <ReportView />
    </ErrorBoundary>
  );
}
