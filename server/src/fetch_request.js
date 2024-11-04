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
    globalThis.result = { error: { message: e.message, stack: e.stack } };
}

try {
    const req = new Request($URI, {
        headers: new Headers($HEADERS),
        method: "$HTTP_METHOD",
        // TODO
        // body: ["GET", "HEAD"].includes(method) ? undefined : bytes.buffer,
    });

    let out = object.fetch(req);

    if (out instanceof Promise) {
        out = await out;
    }

    if (out instanceof Response) {
        const body = await out.bytes();
        const headers = [...out.headers.entries()];

        // uint8array to array
        let bytes = Array.from(body);

        globalThis.result = { response: { bytes, headers } };
    }
} catch (e) {
    globalThis.result = { error: { message: e.message, stack: e.stack } };
}
