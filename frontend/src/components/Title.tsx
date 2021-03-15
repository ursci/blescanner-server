import React from "react";
import Typography from "@material-ui/core/Typography";

type TitleProps = {
  title: string;
};

export const Title: React.FC<TitleProps> = ({ title }: TitleProps) => {
  return (
    <Typography component="h2" variant="h6" color="primary" gutterBottom>
      {title}
    </Typography>
  );
};
