import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: 'selector',
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
      colors: {
        'primary-light': '#F8F5F1',
        'secondary-light': '#DDE4E2',
        'primary-dark': '#323447',
        'secondary-dark': '#3C3D50',
        'accent-green': '#28776C',
        'accent-purple': '#AD2850',
      },
    },
  },
  plugins: [],
};
export default config;
