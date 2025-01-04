import { Card, createTheme } from '@mantine/core';

export const theme = createTheme({
  breakpoints: {
    xs: '36em',
    sm: '48em',
    md: '62em',
    lg: '75em',
    xl: '88em',
  },

  components: {
    Card: Card.extend({
      defaultProps: {
        padding: 'lg',
        shadow: 'md',
        radius: 'xl',
        withBorder: true,
      },
    }),
  },

  colors: {
    // Dark mode primary (base: #323447)
    dark: [
      '#D0D0D0', // text
      '#A0A0A0', // text (in report page public/private)
      '#9A9A9A', // links in the footer
      '#6A6C7A', // placeholder texts
      '#333544', // card border
      '#232534', 
      //  '#1E202E', // card background (blue-er)
      //  '#191B28', // background (blue-er)
      '#272934', // card background
      '#24262E', // background
      '#141622',
      '#0F111C'
    ],

    accentGreen: [
      '#E8F5F3',
      '#D1EBE7',
      '#B3E0D9',
      '#BAE1DB', // icons in sidebar (dark mode)
      '#8CCDC3', // links in navbar (dark mode)
      '#28776C', 
      '#246B61', // background of icons in sidebar (dark mode)
      '#205F56',
      '#1C534B', // buttons
      '#184740'
    ]
  },

  primaryColor: 'accentGreen', // Set your primary color
});
