import React from 'react';
import { Title } from '@mantine/core';
import LoremIpsum from './LoremIpsum';
import { ScrollToTop } from '../layouts/shared/ScrollToTop.tsx';

const Demo: React.FC = () => {
  return (
    <>
      <Title order={2}>Report request</Title>
      <LoremIpsum n={20} />
      <ScrollToTop />
    </>
  );
};

export default Demo;
