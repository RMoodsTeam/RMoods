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
    // Light mode primary (base: #F8F5F1)
    primaryLight: [
      '#FFFFFF',
      '#FDFCFB',
      '#FAF8F6',
      '#F8F5F1', // base
      '#E6E3DF',
      '#D4D1CD',
      '#C2BFBB',
      '#B0ADA9',
      '#9E9B97',
      '#8C8985'
    ],

    // Light mode secondary (base: #DDE4E2)
    secondaryLight: [
      '#F5F8F7',
      '#E9EFED',
      '#DDE4E2', // base
      '#C9D2D0',
      '#B5C0BE',
      '#A1AEAC',
      '#8D9C9A',
      '#798A88',
      '#657876',
      '#516664'
    ],

    // Dark mode primary (base: #323447)
    primaryDark: [
      '#4C4E6B',
      '#3F415A',
      '#323447', // base
      '#2D2F41',
      '#282A3B',
      '#232534',
      '#1E202E',
      '#191B28',
      '#141622',
      '#0F111C'
    ],

    // Dark mode secondary (base: #3C3D50)
    secondaryDark: [
      '#565874',
      '#494B63',
      '#3C3D50', // base
      '#363748',
      '#303140',
      '#2A2B38',
      '#242530',
      '#1E1F28',
      '#181920',
      '#121318'
    ],

    accentGreen: [
      '#E8F5F3',
      '#D1EBE7',
      '#BAE1DB',
      '#A3D7CF',
      '#8CCDC3',
      '#28776C', // base
      '#246B61',
      '#205F56',
      '#1C534B',
      '#184740'
    ],

    // Accent purple (base: #AD2850)
    accentPurple: [
      '#FCE8EE',
      '#F9D1DD',
      '#F6BACC',
      '#F3A3BB',
      '#F08CAA',
      '#AD2850', // base
      '#9C2448',
      '#8B2040',
      '#7A1C38',
      '#691830'
    ]
  },

  primaryColor: 'accentGreen', // Set your primary color
});
