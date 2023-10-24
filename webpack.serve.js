const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

/**
 * @type { import("webpack").Configuration }
 */
module.exports = {
  entry: './index.js',
  devServer: {
    static: {
      directory: path.join(__dirname, 'public')
    },
    client: {
      overlay: false,
    }
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './index.html'
    }),
  ],
  experiments: {
    futureDefaults: true,
  },
  mode: 'development',
};
