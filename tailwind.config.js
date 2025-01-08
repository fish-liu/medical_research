/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,svelte,js,ts}'],
  safelist:[
    {
      pattern: /^justify-/,
    },
    {
      pattern: /^items-/,
    }
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

