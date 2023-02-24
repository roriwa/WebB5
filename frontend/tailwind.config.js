module.exports = {
    content: ['./src/**/*.{html,js, vue}'],
    plugins: [
        function ({addVariant}) {
            addVariant('child', '& > *');
            addVariant('child-hover', '& > *:hover');
        }
    ],
}
