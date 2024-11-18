import React, {useState} from "react";
import {Box, Button, Center, Code, Title} from "@mantine/core";
import {FeedRequest} from "../rmoods/client/types";
import RMoodsClient from "../rmoods/client/RMoodsClient.ts";
import LoremIpsum from "./LoremIpsum";

const Demo: React.FC = () => {
  const [reportRes, setReportRes] = useState<object | null>(null);
  const reportRequest: FeedRequest = {
    resourceKind: 'userPosts',
    reportTypes: ['language'],
    dataSources: [
      {
        name: 'spez',
        share: 100
      },
    ],
    size: 10,
    sorting: {
      kind: 'top',
      time: 'all'
    }
  }

  const onClick = () => {
    console.log(JSON.stringify(reportRequest, null, 2));
    RMoodsClient.requestReport(reportRequest).then((res) => {
      console.log('Received report response')
      setReportRes(res)
    });
  }

  return (
    <>
      <Title order={2}>Report request</Title>
      <Box>
        <Button onClick={onClick}>Request Report</Button>
        <Center>
          Response:
          <Code>
            {JSON.stringify(reportRes, null, 2)}
          </Code>
        </Center>
      </Box>
      <LoremIpsum n={3} />
    </>
  );
};

export default Demo;