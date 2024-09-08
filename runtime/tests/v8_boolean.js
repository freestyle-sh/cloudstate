{
    const base = true;
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expected = true;
    const object = getRoot("test-root");
    if (!object) {
        throw new Error("object should exist");
    }
    if (object.value === undefined) {
        throw new Error("object.value should exist");
    }
    if (typeof object.value !== "boolean") {
        throw new Error("object.value should be a boolean");
    }
    if (object.value !== expected) {
        throw new Error(
            `Expected ${JSON.stringify(expected)}, got ${
                JSON.stringify(object.value)
            }`,
        );
    }

    object.value = !object.value;
    commit();
}

// END_FILE

{
    const expected = false;
    const object = getRoot("test-root");
    if (!object) {
        throw new Error("object should exist");
    }
    if (object.value === undefined) {
        throw new Error("object.value should exist");
    }
    if (typeof object.value !== "boolean") {
        throw new Error("object.value should be a boolean");
    }
    if (object.value !== expected) {
        throw new Error(
            `Expected ${JSON.stringify(expected)}, got ${
                JSON.stringify(object.value)
            }`,
        );
    }
}
