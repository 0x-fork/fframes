import reactRefresh from "@vitejs/plugin-react-refresh";
import { defineConfig } from "vite";
import ViteRsw from "vite-plugin-rsw";

export default defineConfig({
  assetsInclude: ["./media/*", "fframes-editor/*.wasm"],
  server: {
    fs: {
      strict: false,
    },
  },
  plugins: [
    reactRefresh(),
    ViteRsw({
      profile: "dev",
      crates: ["bind"],
    }),
  ],
});
