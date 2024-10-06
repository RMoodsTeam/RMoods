import { atom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { RMoodsColorMode } from "./theme";

export const userInfoAtom = atom<any>({});

export const colorModeAtom = atomWithStorage<RMoodsColorMode>(
  "COLOR_MODE",
  (localStorage.getItem("COLOR_MODE") as RMoodsColorMode) || "light",
);
