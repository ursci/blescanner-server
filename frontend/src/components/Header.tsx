import React, { FunctionComponent, Fragment } from "react";
import {
  AppBar,
  Toolbar,
  Typography,
  makeStyles,
  Link,
} from "@material-ui/core";

const useStyle = makeStyles((theme) => ({
  mcrLogoText: {
    color: "#6495ed",
    font: "Noto Sans",
    margin: theme.spacing(1),
    flexGrow: 1,
  },
}));

export const Header: FunctionComponent = () => {
  const classes = useStyle();

  return (
    <Fragment>
      <AppBar color="default" position="fixed">
        <Toolbar>
          <Typography variant="h6" className={classes.mcrLogoText}>
            <Link
              href="/"
              style={{
                textDecoration: "none",
                color: "#191970",
              }}
            >
              BLESCANNER
            </Link>
          </Typography>
        </Toolbar>
      </AppBar>
    </Fragment>
  );
};
