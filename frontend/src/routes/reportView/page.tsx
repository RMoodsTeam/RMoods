import {
  Accordion,
  Card,
  Combobox,
  Flex,
  Stack,
  Text,
  Title,
} from '@mantine/core';
import { ErrorBoundary } from 'react-error-boundary';
import { PageFallback } from '../PageFallback.tsx';
import styles from './page.module.scss';
import { RMoodsClient } from '../../rmoods/client/RMoodsClient.ts';
import { useQuery } from '@tanstack/react-query';
import Header = Combobox.Header;
import dayjs from 'dayjs';

const fetchReports = async (params: URLSearchParams) => {
  const queryResponse = await RMoodsClient.fetchUserReports(params);
  return queryResponse;
};

const ReportElement = ({ value, description }) => {
  return (
    <Card className={styles.item}>
      <Accordion radius="lg">
        <Accordion.Item value={value} className={styles.accordion}>
          <Accordion.Control>{value}</Accordion.Control>
          <Accordion.Panel className={styles.accordionInner}>
            {description}
          </Accordion.Panel>
        </Accordion.Item>
      </Accordion>
    </Card>
  );
};

const ReportView = () => {
  const { data, isLoading, error } = useQuery({
    queryKey: ['reportsView', '?per_page=30&mine=true&page=1'],
    queryFn: () =>
      fetchReports(new URLSearchParams('?per_page=30&mine=true&page=1')),
  });

  const reportMetadata = data?.reports[1];

  const firstReport = data?.reports[1].analyses
    ? Object.entries(data.reports[1].analyses)
    : [];

  return (
    // <Stack className={styles.wrapper}>
    //   <Title order={1}>{reportMetadata?.title} </Title>
    //   <Text> {reportMetadata?.description}</Text>
    //   <Text>
    //     {' '}
    //     Created at{' '}
    //     {dayjs(reportMetadata?.created_at).format('YYYY-MM-DD HH:mm')}
    //   </Text>
    //   {firstReport.map((item) => {
    //     console.log(item[0]);
    //     return (
    //       <ReportElement
    //         key={item[0]}
    //         value={item[0]}
    //         description={JSON.stringify(item[1], null, 2)}
    //       />
    //     );
    //   })}
    // </Stack>
    <Stack className={styles.wrapper}>
      <Title order={1}> {reportMetadata?.title} </Title>
      <Text> Description </Text>
      <Text>
        {' '}
        Created at{' '}
        {dayjs(reportMetadata?.created_at).format('YYYY-MM-DD HH:mm')}
      </Text>
      {firstReport.map((item) => {
        console.log(item[0]);
        return (
          <ReportElement
            key={item[0]}
            value={item[0]}
            description={JSON.stringify(item[1], null, 2)}
          />
        );
      })}
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
