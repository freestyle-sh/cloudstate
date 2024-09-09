{
    const base = "hello, world!";
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expected = "hello, world!";
    const object = getRoot("test-root");
    if (!object) {
        throw new Error("object should exist");
    }
    if (object.value === undefined) {
        throw new Error("object.value should exist");
    }
    if (typeof object.value !== "string") {
        throw new Error("object.value should be a string");
    }
    if (object.value !== expected) {
        throw new Error(
            `Expected ${JSON.stringify(expected)}, got ${
                JSON.stringify(object.value)
            }`,
        );
    }

    object.value = object.value.replace("world", "cloudstate");
    commit();
}

// END_FILE

{
    const expected = "hello, cloudstate!";
    const object = getRoot("test-root");
    if (!object) {
        throw new Error("object should exist");
    }
    if (object.value === undefined) {
        throw new Error("object.value should exist");
    }
    if (typeof object.value !== "string") {
        throw new Error("object.value should be a string");
    }
    if (object.value !== expected) {
        throw new Error(
            `Expected ${JSON.stringify(expected)}, got ${
                JSON.stringify(object.value)
            }`,
        );
    }
}
