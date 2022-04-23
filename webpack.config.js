const HtmlWebPackPlugin = require("html-webpack-plugin");
const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./frontend/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new HtmlWebPackPlugin({
      template: path.resolve( __dirname, 'frontend/public/index.html' ),
      filename: 'index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  module: {
    rules: [{
      test: /\.js$/,
      exclude: /node_modules/,
      use: {
        loader: 'babel-loader',
        options: {
          presets: ['@babel/preset-env', '@babel/preset-react']
        }
      }
    }]
  }
};
