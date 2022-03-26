const colors = require("tailwindcss/colors");

module.exports = {
  darkMode: 'class',
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {},
    colors: {
      primary: {
        50: "#edf5ff",
        100: "#d0e2ff",
        200: "#a6c8ff",
        300: "#78a9ff",
        400: "#4589ff",
        500: "#0f62fe",
        600: "#0043ce",
        700: "#002d9c",
        800: "#001d6c",
        900: "#001141",
      },
      secondary: {
        50: colors.zinc["300"],
        100: colors.zinc["400"],
        200: colors.zinc["500"],
        300: colors.zinc["600"],
        400: colors.zinc["700"],
        500: colors.zinc["800"],
        600: colors.zinc["900"],
      },
      warning: colors.amber,
      danger: {
        50: colors.red["100"],
        100: colors.red["200"],
        200: colors.red["300"],
        300: colors.red["400"],
        400: colors.red["500"],
        500: colors.red["600"],
        600: colors.red["700"],
        700: colors.red["800"],
        800: colors.red["900"],
      },
      background: {
        400: colors.gray["50"],
        500: colors.gray["100"],
        600: colors.gray["200"],
        700: colors.gray["300"],
        800: colors.gray["400"],
        900: colors.gray["500"],
      },
      darkbackground: {
        300: colors.gray["500"],
        400: colors.gray["600"],
        500: colors.gray["700"],
        600: colors.gray["800"],
        700: colors.gray["900"],
      },
      graphics: {
        1: "#0f62fe",
        2: colors.lime["500"],
        red: {
          300: colors.red["200"],
          400: colors.red["300"],
          500: colors.red["400"],
          600: colors.red["500"],
          700: colors.red["600"],
        },
        green: {
          300: colors.green["200"],
          400: colors.green["300"],
          500: colors.green["400"],
          600: colors.green["500"],
          700: colors.green["600"],
        },
        yellow: {
          300: colors.yellow["100"],
          400: colors.yellow["200"],
          500: colors.yellow["300"],
          600: colors.yellow["400"],
          700: colors.yellow["600"],
        }
      },
      white: colors.white,
      light: colors.gray["100"],
      dark: colors.gray["900"],
      black: colors.black
    }
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
