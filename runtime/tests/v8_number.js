{
    const base = 42;
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expected = 42;
    const object = getRoot("test-root");
    if (!object) {
        throw new Error("object should exist");
    }
    if (object.value === undefined) {
        throw new Error("object.value should exist");
    }
    if (typeof object.value !== "number") {
        throw new Error("object.value should be a number");
    }
    if (object.value !== expected) {
        throw new Error(
            `Expected ${JSON.stringify(expected)}, got ${
                JSON.stringify(object.value)
            }`,
        );
    }

    if (object.value++ !== expected) {
        throw new Error("object.value should be the same as base");
    }
    if (++object.value !== expected + 2) {
        throw new Error("object.value should be the base + 2");
    }
    commit();
}

// END_FILE

{
    const expected = 44;
    const object = getRoot("test-root");
    if (!object) {
        throw new Error("object should exist");
    }
    if (object.value === undefined) {
        throw new Error("object.value should exist");
    }
    if (typeof object.value !== "number") {
        throw new Error("object.value should be a number");
    }
    if (object.value !== expected) {
        throw new Error(
            `Expected ${JSON.stringify(expected)}, got ${
                JSON.stringify(object.value)
            }`,
        );
    }
}
