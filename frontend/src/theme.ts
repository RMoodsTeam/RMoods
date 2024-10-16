import { extendTheme, type ThemeConfig } from '@chakra-ui/react'

export type RMoodsColorMode = 'light' | 'dark' | 'system';
export const DEFAULT_COLOR_MODE: RMoodsColorMode = localStorage.getItem('COLOR_MODE') as RMoodsColorMode || 'light';

export const getSystemMode = (): 'light' | 'dark' => {
  if (window.matchMedia) {
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      return 'dark'
    }
  }
  return 'light'
}

if (!localStorage.getItem("COLOR_MODE")) {
  localStorage.setItem("COLOR_MODE", DEFAULT_COLOR_MODE);
}
const config: ThemeConfig = {
  initialColorMode: localStorage.getItem('COLOR_MODE') as RMoodsColorMode,
  useSystemColorMode: false,
  disableTransitionOnChange: false,
}

export const rmoodsTheme = extendTheme({
  components: {
    Card: {
      baseStyle: {
        container: {
          padding: 4,
          margin: 4,
        }
      }
    }
  },
  config, styles: {
    global: {
      body: {
        transitionProperty: 'background-color',
        transitionDuration: '0.3s',
      }
    }
  }
})
