const path = require("path");
const withCSS = require("@zeit/next-css");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = () => withCSS({
  webpack: config => ({
    ...config,
    resolve: {
      ...config.resolve,
      alias: {
        ...config.resolve.alias,
        "@": path.resolve(__dirname, "./"),
      },
    },
    plugins: [
      ...config.plugins,
      // new WasmPackPlugin({
      // https://rustwasm.github.io/wasm-pack/book/commands/build.html
      //   crateDirectory: path.resolve(__dirname, "../rust_mal_lib_wasm"),
      // }),
    ],
    node: {
      fs: "empty", // Fixes npm packages that depend on `fs` module
    },
  }),
});
