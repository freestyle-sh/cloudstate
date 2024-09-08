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
    let iterations = 0;
    const iterator = object.value[Symbol.iterator]();
    for (const [key, value] of iterator) {
        if (expected.get(key) !== value) {
            throw new Error(
                `Expected ${JSON.stringify(expected.get(key))}, got ${
                    JSON.stringify(value)
                }`,
            );
        }
        if (object.value.get(key) !== value) {
            throw new Error(
                `Expected ${JSON.stringify(object.value.get(key))}, got ${
                    JSON.stringify(value)
                }`,
            );
        }
        iterations++;
    }
    if (iterations !== expected.size) {
        throw new Error(`Expected ${expected.size} entries, got ${iterations}`);
    }
}
