import { Flex, Image } from '@mantine/core';
import React from 'react';
import classes from './ProfilePictureFallback.module.scss';

export const ProfilePictureFallback = ({ error }) => {
  console.error('ProfilePictureFallback:', error.message);
  return (
    <Flex className={classes.flex}>
      <Image
        src={'https://placehold.co/250x250?text=PfP'}
        alt={`defaulting to placeholder: ${error.message}`}
        w={'250px'}
        h={'250px'}
        fit="contain"
        radius="50%"
        referrerPolicy="no-referrer"
        className={classes.image}
      />
    </Flex>
  );
};
