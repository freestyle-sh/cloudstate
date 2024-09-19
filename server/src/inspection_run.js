globalThis.process = {
    env: env_string,
};

const classes = await import("./lib.js").catch((e) => {
    console.error("Error importing classes", e);
    throw e;
});

// temporary hack to be compatible with legacy freestyle apis
globalThis.requestContext = {
    getStore: () => {
        return {
            env: {
                invalidateMethod: (rawMethod) => {
                    const method = rawMethod.toJSON();
                    fetch(
                        `invalidate_endpoint/${method.instance}/${method.method}`,
                        {
                            method: "POST",
                            headers: {
                                "Content-Type": "application/json",
                            },
                        },
                    ).catch((e) => {
                        console.error(e);
                    });
                },
            },
        };
    },
};

for (const className of Object.keys(classes)) {
    const klass = classes[className];
    registerCustomClass(klass);
}

try {
    globalThis.result = {
        result: await (async function () {
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
