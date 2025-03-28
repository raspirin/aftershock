/** @type {import('tailwindcss').Config} */
    module.exports = {
      content: {
        relative: true,
        files: ["*.html", "./crates/aftershock/src/**/*.rs"],
        transform: {
          rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
      },
      theme: {
        extend: {
          fontFamily: {
            sans: [
              'system-ui',
              '-apple-system',
              'Segoe UI',
              'Roboto',
              'Helvetica Neue',
              'Arial',
              'Noto Sans SC',
              'sans-serif'
            ],
            
            serif: [
              'Merriweather',
              'Georgia',
              'Cambria',
              'Times New Roman',
              'Noto Serif SC',
              'serif'
            ]
          },
        }
      },
      plugins: [require("@tailwindcss/typography")],
    }
    