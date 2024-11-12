/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{js,ts,jsx,tsx}', './src-tauri/templates/*.html'],
  theme: {
    extend: {
      colors: {
        bgblack: '#1B2021',
      },
    },
  },
  plugins: [],
};
