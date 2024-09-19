globalThis.process = {
    env: $ENV_STRING,
};

const classes = await import("./lib.js").catch((e) => {
    console.error("Error importing classes", e);
    throw e;
});

for (const className of Object.keys(classes)) {
    const klass = classes[className];
    registerCustomClass(klass);
}

// temporary hack to be compatible with legacy freestyle apis
globalThis.requestContext = {
    getStore: () => {
        return {
            request: new Request($URI, {
                headers: new Headers($HEADERS),
            }),
            env: {
                invalidateMethod: (rawMethod) => {
                    const method = rawMethod.toJSON();
                    fetch(
                        `$INVALIDATE_ENDPOINT/${method.instance}/${method.method}`,
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

let object;

try {
    object = getRoot($ID) || getCloudstate($ID);
} catch (e) {
    console.error("Error getting root or cloudstate", e);
    throw e;
}

try {
    if (!object) {
        globalThis.result = { error: { message: "Object not found" } };
    } else if (!object[$METHOD]) {
        globalThis.result = {
            error: {
                message: `Method not found on class ${
                    object?.constructor?.name ?? "unknown"
                }`,
            },
        };
    } else {
        globalThis.result = {
            result: await object[$METHOD](...JSON.parse(`$PARAMS`)),
        };
    }
} catch (e) {
    globalThis.result = { error: { message: e.message, stack: e.stack } };
}
