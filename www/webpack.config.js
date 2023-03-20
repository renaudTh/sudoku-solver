const path = require("path");
module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
    topLevelAwait: true,
  },
    devServer: {
    static: {
      directory: path.join(__dirname, './'),
    },
    compress: true,
    port: 8000
  }
};
