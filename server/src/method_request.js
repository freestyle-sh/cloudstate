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
            result: await object[$METHOD](...deserializeJsonWithBlobs($PARAMS)),
        };
    }
} catch (e) {
    globalThis.result = { error: { message: e.message, stack: e.stack } };
}

export function deserializeJsonWithBlobs(jsonString) {
    const obj = JSON.parse(jsonString);

    const deserializeBlob = ({ mimeType, data }) => {
        const byteCharacters = atob(data);
        const byteNumbers = new Array(byteCharacters.length);
        for (let i = 0; i < byteCharacters.length; i++) {
            byteNumbers[i] = byteCharacters.charCodeAt(i);
        }
        const byteArray = new Uint8Array(byteNumbers);
        return new Blob([byteArray], { type: mimeType });
    };

    const deserializeObject = (obj) => {
        if (Array.isArray(obj)) {
            return obj.map((item) => deserializeObject(item));
        }
        if (obj && typeof obj === "object" && obj.__isDate) {
            return new Date(obj.dateString);
        }
        if (obj && typeof obj === "object" && obj.__isBlob) {
            return deserializeBlob(obj);
        }

        if (obj !== null && typeof obj === "object") {
            const entries = Object.entries(obj).map(([key, value]) => {
                if (value && typeof value === "object" && value.__isBlob) {
                    //@ts-ignore
                    return [key, deserializeBlob(value)];
                } else if (
                    value && typeof value === "object" && value.__isDate
                ) {
                    return [key, new Date(value.dateString)];
                } else if (value && typeof value === "object") {
                    return [key, deserializeObject(value)];
                }
                return [key, value];
            });
            return Object.fromEntries(entries);
        }
        return obj;
    };

    return deserializeObject(obj);
}
