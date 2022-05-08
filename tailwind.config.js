module.exports = {
  /* jit: true, */
  purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./src/**/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
            //"./static/**/*.css",
        ],
  },
  darkMode: process.env.NODE_ENV == "production" ? "media" : "class",
  theme: {
    extend: {
       fontFamily: {
        'noto-sans': ['"Noto Sans"', 'sans-serif'],
        'noto-sans-display': ['"Noto Sans Display"', 'sans-serif'],
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
