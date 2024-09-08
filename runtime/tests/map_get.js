{
    const base = new Map([
        ["foo", 1],
        ["bar", 2],
        ["baz", 3],
    ]);
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expected = new Map([
        ["foo", 1],
        ["bar", 2],
        ["baz", 3],
    ]);

    const object = getRoot("test-root");

    // Test keys that should exist
    for (const [key, value] of expected.entries()) {
        if (object.value.get(key) !== value) {
            throw new Error(
                `Expected ${JSON.stringify(value)}, got ${
                    JSON.stringify(object.value.get(key))
                }`,
            );
        }
    }

    // Test keys that don't exist
    if (object.value.get("qux") !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get("qux"))
            }`,
        );
    }
    if (object.value.get(0) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(undefined))
            }`,
        );
    }
    if (object.value.get(null) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(null))
            }`,
        );
    }
    if (object.value.get(0) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(0))
            }`,
        );
    }
    if (object.value.get(false) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(false))
            }`,
        );
    }
    if (object.value.get("") !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(""))
            }`,
        );
    }
    if (object.value.get(Symbol()) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(Symbol()))
            }`,
        );
    }
    if (object.value.get({}) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get({}))
            }`,
        );
    }
    if (object.value.get([]) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get([]))
            }`,
        );
    }
    if (object.value.get(() => {}) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(() => {}))
            }`,
        );
    }
    if (object.value.get(object) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(object))
            }`,
        );
    }
}
