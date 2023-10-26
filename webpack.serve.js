const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const ASSET_PATH = `${process.env.PUBLIC_PATH || ''}/`;
console.log(ASSET_PATH);

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
    publicPath: ASSET_PATH
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './index.html',
      templateParameters: {
        ga: ASSET_PATH !== '/'
      }
    }),
    new webpack.DefinePlugin({
      'process.env.ASSET_PATH': JSON.stringify(ASSET_PATH),
    }),
  ],
  experiments: {
    futureDefaults: true,
  },
  mode: 'development',
};
