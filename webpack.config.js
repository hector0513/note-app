const path =require("path")
const MiniCssExtractPlugin = require("mini-css-extract-plugin")
module.exports = {
  entry: __dirname + '/src/index.js',
  output: { path: __dirname + '/dist', filename: 'index.js' },
  plugins: [
    new MiniCssExtractPlugin()
  ],
  module: {
    rules: [
      {
        test: /\.css|\.s(c|a)ss$/,
        use:[ {
          loader: MiniCssExtractPlugin.loader,
          options: {
            publicPath: (resourcePath, context) => {
              return path.join(__dirname + 'static');
            },
          },
        },'css-loader', 'sass-loader', {
          loader: 'postcss-loader',
          options: {
            ident: 'postcss',
            plugins: [
              require('tailwindcss'),
              require('autoprefixer'),
            ],
          },
        }],
      },
    ],
  },
};
