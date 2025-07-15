import rust from "@wasm-tool/rollup-plugin-rust";

export default {
  input: {
    vp_js_compat: "../../Cargo.toml", // in a real world example use git submodules
  },
  output: {
    dir: "dist/js",
    format: "es",
    sourcemap: true,
  },
  plugins: [
    rust({
      verbose: true,
    }),
  ],
};
