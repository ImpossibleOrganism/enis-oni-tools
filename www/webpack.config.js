const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: "development",
    experiments: {
        futureDefaults: true,
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './static/index.html',
            inject: 'body',
        }),
    ],
    devServer: {
        compress: true,
        port: 9000,
    },
};
