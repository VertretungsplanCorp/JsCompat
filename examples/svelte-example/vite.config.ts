import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import rust from "@wasm-tool/rollup-plugin-rust";

export default defineConfig({
  plugins: [
    rust({
      verbose: true,
    }),
    wasm(),
    topLevelAwait(),
    sveltekit(),
  ],
});
