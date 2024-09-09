{
    const base = ["a", "b", "c", "d", "e"];
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expectedArr = ["a", "b", "c", "d", "e"];

    const object = getRoot("test-root");
    if (object.value.length !== expectedArr.length) {
        throw new Error(
            `Expected length to be ${expectedArr.length}, got ${object.value.length}`,
        );
    }

    const shifted = object.value.shift();
    if (shifted !== "a") {
        throw new Error(`Expected "a", got ${shifted}`);
    }
    commit();
}

// END_FILE

{
    const expectedArr = ["b", "c", "d", "e"];

    const object = getRoot("test-root");
    if (object.value.length !== expectedArr.length) {
        throw new Error(
            `Expected length to be ${expected.length}, got ${object.value.length}`,
        );
    }
    for (const [i, expected] of expectedArr.entries()) {
        if (object.value[i] !== expected) {
            throw new Error(
                `Expected ${expected} at index ${i}, got ${object.value[i]}`,
            );
        }
    }

    // clear the array
    object.value = [];
    commit();
}

// END_FILE

{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    // shift from an empty array
    const shifted = object.value.shift();
    if (shifted !== undefined) {
        throw new Error(`Expected undefined, got ${shifted}`);
    }
    commit();
}
