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
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "../rust_mal_lib_wasm"),

        // Check https://rustwasm.github.io/wasm-pack/book/commands/build.html for
        // the available set of arguments.
        //
        // Default arguments are `--typescript --target browser --mode normal`.
        // extraArgs: "",

        // Optional array of absolute paths to directories, changes to which
        // will trigger the build.
        // watchDirectories: [
        //   path.resolve(__dirname, "another-crate/src")
        // ],

        // The same as the `--out-dir` option for `wasm-pack`
        // outDir: "pkg",

        // The same as the `--out-name` option for `wasm-pack`
        // outName: "index",

        // If defined, `forceWatch` will force activate/deactivate watch mode for
        // `.rs` files.
        //
        // The default (not set) aligns watch mode for `.rs` files to Webpack's
        // watch mode.
        // forceWatch: true,

        // If defined, `forceMode` will force the compilation mode for `wasm-pack`
        //
        // Possible values are `development` and `production`.
        //
        // the mode `development` makes `wasm-pack` build in `debug` mode.
        // the mode `production` makes `wasm-pack` build in `release` mode.
        // forceMode: "development",
      }),
    ],
    node: {
      fs: "empty", // Fixes npm packages that depend on `fs` module
    },
  }),
});
