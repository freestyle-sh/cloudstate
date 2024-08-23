deno_core::extension!(
    superjson,
    esm_entry_point = "ext:superjson/superjson.js",
    esm = [ dir "src/extensions", "superjson.js" ],
);
