import React, { Fragment } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
  makeStyles,
} from "@material-ui/core";
import Link from "@material-ui/core/Link";

import { Title } from "~/components/Title";
import { DeviceLog } from "~/models/device_logs";

const useStyles = makeStyles((theme) => ({
  seeMore: {
    marginTop: theme.spacing(3),
  },
}));

type DashboardProps = {
  deviceLogs: Array<DeviceLog>;
};

export const Dashboard: React.FC<DashboardProps> = ({
  deviceLogs,
}: DashboardProps) => {
  const classes = useStyles();

  return (
    <Fragment>
      <Title title={"Blescanner dashboard"} />
      <Table size="small">
        <TableHead>
          <TableRow>
            <TableCell>Scanned Time</TableCell>
            <TableCell>Gateway Name</TableCell>
            <TableCell>Location</TableCell>
            <TableCell>Device UUID</TableCell>
            <TableCell>Device Name</TableCell>
            <TableCell>RSSI</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {deviceLogs.map((devicelog) => (
            <TableRow key={devicelog.id}>
              <TableCell>{devicelog.scanned_time}</TableCell>
              <TableCell>{devicelog.gateway_name}</TableCell>
              <TableCell>{devicelog.location}</TableCell>
              <TableCell>{devicelog.device_uuid}</TableCell>
              <TableCell>
                {devicelog.device_name ?? "デバイス名は空欄です"}
              </TableCell>
              <TableCell>{devicelog.rssi}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <div className={classes.seeMore}>
        <Link color="primary" href="#">
          See more logs
        </Link>
      </div>
    </Fragment>
  );
};
