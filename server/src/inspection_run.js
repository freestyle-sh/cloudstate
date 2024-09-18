globalThis.process = {
    env: env_string,
};

const classes = await import("./lib.js").catch((e) => {
    console.error("Error importing classes", e);
    throw e;
});

for (const className of Object.keys(classes)) {
    const klass = classes[className];
    registerCustomClass(klass);
}

try {
    globalThis.result = {
        result: (function () {
            run_script;
        })(),
    };
} catch (e) {
    globalThis.result = {
        error: {
            message: e.message,
            stack: e.stack,
        },
    };
}
