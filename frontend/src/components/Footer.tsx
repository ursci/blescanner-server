import React, { FunctionComponent } from "react";
import {
  BottomNavigation,
  BottomNavigationAction,
  makeStyles,
} from "@material-ui/core";
import { FooterItemLink } from "~/models/components";

const useStyle = makeStyles(() => ({
  footer: {
    width: "100%",
    backgroundColor: "#f5f5f5",
    position: "fixed",
    bottom: 0,
    left: 0,
  },
}));

export const Footer: FunctionComponent = () => {
  const classes = useStyle();

  const menuItems: Array<FooterItemLink> = [
    { label: "About", value: "about", href: "/about" },
    { label: "利用規約", value: "terms", href: "/terms" },
  ];

  return (
    <BottomNavigation showLabels className={classes.footer}>
      {menuItems.map((item, i) => (
        <BottomNavigationAction
          key={i}
          label={item.label}
          value={item.value}
          href={item.href}
        />
      ))}
    </BottomNavigation>
  );
};
