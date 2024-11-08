import React from "react";
import {Card, Title} from "@mantine/core";

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

const Section: React.FC<SectionProps> = ({title, content}) => {
  return (
    <Card>
      <Title order={3}>{title}</Title>
      <p>{content}</p>
    </Card>
  );
};

export default Section;
