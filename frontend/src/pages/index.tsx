import Head from "next/head";
import React, { Fragment, ReactElement, useState, useEffect } from "react";

import { Layout } from "~/components/Layout";
import { Dashboard } from "~/components/Dashboard";
import { DeviceLog } from "~/models/device_logs";
import { DeviceLogUseCase } from "~/usecases/device_logs";
import { DeviceLogRepository } from "~/repositories/device_logs";

const Index = (): ReactElement => {
  // usecaseクラスの生成
  const deviceLogUseCase = new DeviceLogUseCase(new DeviceLogRepository());

  // Hooks
  const [deviceLogs, setDeviceLogs] = useState<Array<DeviceLog>>([]);

  // blescanner-serverからデータを取ってくる
  useEffect(() => {
    const fetchRequiredData = async (): Promise<void> => {
      const responses = await deviceLogUseCase.fetchDeviceLogs();
      setDeviceLogs(responses);
    };
    fetchRequiredData();
  }, []);

  return (
    <Fragment>
      <Head>
        <title>Blescanner Admin Dashboard</title>
      </Head>
      <Layout>
        <Dashboard deviceLogs={deviceLogs} />
      </Layout>
    </Fragment>
  );
};

export default Index;
