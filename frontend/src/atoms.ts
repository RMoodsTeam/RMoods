import { atom } from 'jotai';
import { atomWithStorage } from 'jotai/utils';
import { MantineColorScheme } from '@mantine/core';
import { User } from './rmoods/types.ts';

export const userInfoAtom = atom<User | null>(null);

export const colorModeAtom = atomWithStorage<MantineColorScheme>(
  'COLOR_MODE',
  (localStorage.getItem('COLOR_MODE') as MantineColorScheme) || 'light'
);

export const wsConnectionStatusAtom = atom<boolean>(false);
