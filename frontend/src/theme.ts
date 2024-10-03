import { extendTheme, type ThemeConfig } from '@chakra-ui/react'

const config: ThemeConfig = {
  initialColorMode: 'light',
  useSystemColorMode: false,
  disableTransitionOnChange: false,
}

const rmoodsTheme = extendTheme({
  config, styles: {
    global: {
      body: {
        transitionProperty: 'background-color',
        transitionDuration: '0.2s',
      }
    }
  }
})

export default rmoodsTheme;
