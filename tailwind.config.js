/** @type {import('tailwindcss').Config} */
export default {
    content: {
        files: [
            '*.html',
            './src/**/*.{rs,html}',
        ],
    },
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/typography'),
    ],
}
