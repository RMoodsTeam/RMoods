import { Card, Typography } from "@mui/joy";
import React from "react";

/**
 * Section component, used to display a section with a title and content.
 * @param title - The title of the section.
 * @param content - The content of the section.
 * @returns Element
 */
interface SectionProps {
  title: string;
  content: string;
}

const Section: React.FC<SectionProps> = ({ title, content }) => {
  return (
    <Card>
      <Typography level="h3">{title}</Typography>
      <p>{content}</p>
    </Card>
  );
};

export default Section;
