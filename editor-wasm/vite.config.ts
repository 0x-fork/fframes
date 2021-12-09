import reactRefresh from "@vitejs/plugin-react-refresh";
import { defineConfig } from "vite";
import ViteRsw from "vite-plugin-rsw";

export default defineConfig({
  server: {
    fs: {
      strict: false,
    },
  },
  plugins: [
    reactRefresh(),
    ViteRsw({
      crates: ["bind"],
    }),
  ],
});
