// import { serialize } from "npm:superjson";
// serialize;

import { build } from "npm:esbuild";

build({
  entryPoints: ["node_modules/superjson/dist/index.js"],
  outfile: "superjson.js",
  bundle: true,
  format: "esm",
});
