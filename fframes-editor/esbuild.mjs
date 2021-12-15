import esbuild from "esbuild";

esbuild
  .build({
    entryPoints: ["./src/fframes-editor.tsx"],
    outdir: "dist",
    bundle: true,
    splitting: true,
    format: "esm",
    target: ["es2020"],
    loader: { ".url.ts": "file" },
    watch: process.argv.some((arg) => arg.includes("-w")),
  })
  .catch(() => process.exit(1));
