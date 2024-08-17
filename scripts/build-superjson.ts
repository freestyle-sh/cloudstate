// import { serialize } from "npm:superjson";
// serialize;

import { build } from "npm:esbuild";

build({
  entryPoints: ["node_modules/superjson/dist/index.js"],
  outfile: new URL("../src/superjson.js", import.meta.url).pathname,
  bundle: true,
  format: "esm",
  footer: {
    js: "globalThis.SuperJSON = SuperJSON;",
  },
});
