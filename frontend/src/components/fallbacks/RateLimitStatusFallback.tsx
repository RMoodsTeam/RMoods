import { HoverCard } from '@mantine/core';
import { IconBrandReddit } from '@tabler/icons-react';
import React from 'react';

export const RateLimitStatusFallback = () => {
  return (
    <HoverCard>
      <HoverCard.Target>
        <IconBrandReddit size={24} />
      </HoverCard.Target>
      <HoverCard.Dropdown>
        Error occurred while fetching rate limit status
      </HoverCard.Dropdown>
    </HoverCard>
  );
};
