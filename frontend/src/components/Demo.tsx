import React, {useEffect, useState} from "react";
import {Code, Title} from "@mantine/core";
import {FeedRequest} from "../rmoods/client/types";
import RMoodsClient from "../rmoods/client/RMoodsClient.ts";

const Demo: React.FC = () => {
  const [reportRes, setReportRes] = useState<object | null>(null);

  useEffect(() => {
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
        sorting: 'top',
        time: 'week'
      }
    }
    console.log(JSON.stringify(reportRequest, null, 2));
    const fetchReport = async () => {
      return await RMoodsClient.requestReport(reportRequest);
    };
    fetchReport().then((res) => {
      setReportRes(res)
    });
  }, []);

  return (
    <>
      <Title order={1}>Demo</Title>
      <Code>
        {JSON.stringify(reportRes, null, 2)}
      </Code>
    </>
  );
};

export default Demo;