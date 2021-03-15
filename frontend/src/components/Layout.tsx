import { FunctionComponent, Fragment } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";
import { Toolbar } from "@material-ui/core";

export const Layout: FunctionComponent = (props) => (
  <Fragment>
    <Header />
    <Toolbar />
    {props.children}
    <Footer />
  </Fragment>
);
