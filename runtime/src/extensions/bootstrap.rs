deno_core::extension!(
    bootstrap,
    esm_entry_point = "ext:bootstrap/bootstrap.js",
    esm = [ dir "src/extensions", "bootstrap.js" ],
);
